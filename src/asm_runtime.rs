#![allow(unused_macros)]

//! Assembly runtime and support functions for the GBA.

// Note(Lokathor): Functions here will *definitely* panic without the `on_gba`
// cargo feature enabled, and so they should all have the `track_caller`
// attribute set whenever the `on_gba` feature is *disabled*

use crate::gba_cell::GbaCell;

/// Inserts a `nop` instruction.
#[inline(always)]
#[cfg_attr(not(feature = "on_gba"), track_caller)]
pub fn nop() {
  on_gba_or_unimplemented! {
    unsafe {
      core::arch::asm! {
        "nop",
      }
    }
  }
}

/// Atomically swap `x` and the 32-bit value stored at `ptr`.
///
/// ## Safety
/// This both reads and writes `ptr`, so all the usual rules of that apply.
#[inline]
#[cfg_attr(feature = "on_gba", instruction_set(arm::a32))]
#[cfg_attr(not(feature = "on_gba"), track_caller)]
pub unsafe fn swp(mut ptr: *mut u32, x: u32) -> u32 {
  on_gba_or_unimplemented! {
    let output: u32;
    // Note(Lokathor): This won't actually alter the pointer register, but we
    // *tell* LLVM that it will because the pointer register can't be used as
    // the output of the swapping operation.
    #[allow(unused_assignments)]
    unsafe {
      core::arch::asm! {
        "swp {output}, {input}, [{addr}]",
        output = lateout(reg) output,
        input = in(reg) x,
        addr = inlateout(reg) ptr,
      }
    }
    output
  }
}

/// Atomically swap `x` and the 8-bit value stored at `ptr`.
///
/// ## Safety
/// This both reads and writes `ptr`, so all the usual rules of that apply.
#[inline]
#[cfg_attr(feature = "on_gba", instruction_set(arm::a32))]
#[cfg_attr(not(feature = "on_gba"), track_caller)]
pub unsafe fn swpb(mut ptr: *mut u8, x: u8) -> u8 {
  on_gba_or_unimplemented! {
    let output: u8;
    // Note(Lokathor): This won't actually alter the pointer register, but we
    // *tell* LLVM that it will because the pointer register can't be used as
    // the output of the swapping operation.
    #[allow(unused_assignments)]
    unsafe {
      core::arch::asm! {
        "swpb {output}, {input}, [{addr}]",
        output = lateout(reg) output,
        input = in(reg) x,
        addr = inlateout(reg) ptr,
      }
    }
    output
  }
}

#[cfg(target_feature = "thumb-mode")]
macro_rules! a32_code {
  ($($asm_line:expr),+ $(,)?) => {
    concat!(
      ".code 32\n",

      $( concat!($asm_line, "\n") ),+ ,

      ".code 16\n",
    )
  }
}
#[cfg(not(target_feature = "thumb-mode"))]
macro_rules! a32_code {
  ($($asm_line:expr),+ $(,)?) => {
    concat!(
      $( concat!($asm_line, "\n") ),+ ,
    )
  }
}

/// If `on_gba` is enabled, makes a `global_asm` for the function given.
///
/// If `on_gba` is disabled, this does nothing.
macro_rules! global_a32_fn {
  (
    $name:ident [iwram=true] {
      $($asm_line:expr),+ $(,)?
    }
  ) => {
    #[cfg(feature = "on_gba")]
    core::arch::global_asm!{
      a32_code! {
        concat!(".section .iwram.text.", stringify!($name), ", \"x\" "),
        concat!(".global ",stringify!($name)),
        concat!(stringify!($name),":"),
        $( concat!($asm_line, "\n") ),+ ,
        ".pool",
      }
    }
  };
  (
    $name:ident [] {
      $($asm_line:expr),+ $(,)?
    }
  ) => {
    #[cfg(feature = "on_gba")]
    core::arch::global_asm!{
      a32_code! {
        concat!(".section .text.", stringify!($name), ", \"x\" "),
        concat!(".global ",stringify!($name)),
        concat!(stringify!($name),":"),
        $( concat!($asm_line, "\n") ),+ ,
        ".pool",
      }
    }
  };
}

macro_rules! while_swapped {
  (
    ptr=$ptr:literal, val=$val:literal {
      $($asm_line:expr),+ $(,)?
    }
  ) => {
    concat!(
      concat!("swp ",$val,", ",$val,", [",$ptr,"]\n"),

      $( concat!($asm_line, "\n") ),+ ,

      concat!("swp ",$val,", ",$val,", [",$ptr,"]\n"),
    )
  }
}

macro_rules! with_spsr_held_in {
  ($reg:literal, {
    $($asm_line:expr),* $(,)?
  }) => {
    concat!(
      concat!("mrs ", $reg, ", SPSR\n"),
      $( concat!($asm_line, "\n") ),* ,
      concat!("msr SPSR, ", $reg, "\n"),
    )
  }
}

macro_rules! set_cpu_control {
  // CPSR control bits are: `I F T MMMMM`, and T must always be left as 0.
  // * 0b10011: Supervisor (SVC)
  // * 0b11111: System (SYS)
  (System, irq_masked: false, fiq_masked: false) => {
    "msr CPSR_c, #0b00011111\n"
  };
  (Supervisor, irq_masked: true, fiq_masked: false) => {
    "msr CPSR_c, #0b10010010\n"
  };
}

macro_rules! when {
  ($reg:literal == $op2:literal [label_id=$label:literal] {
    $($asm_line:expr),* $(,)?
  }) => {
    concat!(
      concat!("cmp ", $reg, ", ", $op2, "\n"),
      concat!("bne ", $label, "f\n"),
      $( concat!($asm_line, "\n") ),* ,
      concat!($label, ":\n"),
    )
  };
  ($reg:literal != $op2:literal [label_id=$label:literal] {
    $($asm_line:expr),* $(,)?
  }) => {
    concat!(
      concat!("cmp ", $reg, ", ", $op2, "\n"),
      concat!("beq ", $label, "f\n"),
      $( concat!($asm_line, "\n") ),* ,
      concat!($label, ":\n"),
    )
  };
  ($reg:literal >=u $op2:literal [label_id=$label:literal] {
    $($asm_line:expr),* $(,)?
  }) => {
    concat!(
      concat!("cmp ", $reg, ", ", $op2, "\n"),
      concat!("bcc ", $label, "f\n"), // cc: Unsigned LT
      $( concat!($asm_line, "\n") ),* ,
      concat!($label, ":\n"),
    )
  };
  ($reg:literal <=u $op2:literal [label_id=$label:literal] {
    $($asm_line:expr),* $(,)?
  }) => {
    concat!(
      concat!("cmp ", $reg, ", ", $op2, "\n"),
      concat!("bhi ", $label, "f\n"), // hi: Unsigned GT
      $( concat!($asm_line, "\n") ),* ,
      concat!($label, ":\n"),
    )
  };
}

/// Sets `lr` properly and then uses `bx` on the register given.
macro_rules! a32_fake_blx {
  (reg=$reg_name:expr, label_id=$label:expr) => {
    concat!(
      concat!("adr lr, ", $label, "f\n"),
      concat!("bx ", $reg_name, "\n"),
      concat!($label, ":\n"),
    )
  };
}

macro_rules! with_pushed_registers {
  ($reglist:expr, {
    $($asm_line:expr),* $(,)?
  }) => {
    concat!(
      concat!("push ", $reglist, "\n"),
      $( concat!($asm_line, "\n") ),* ,
      concat!("pop ", $reglist, "\n"),
    )
  }
}

global_a32_fn! {_start [] {
  "b 1f",
  ".space 0xE0",
  "1:",

  "mov r12, #0x04000000",

  // Configure WAITCNT to the GBATEK suggested default
  "add r0, r12, #0x204",
  "ldr r1, =0x4317",
  "strh r1, [r0]",

  // TODO: iwram copying

  // TODO: ewram copying

  // TODO: bss zeroing

  // Tell the BIOS about our irq handler
  "ldr r0, =_asm_runtime_irq_handler",
  "str r0, [r12, #-4]",

  // Note(Lokathor): we do a `bx` instead of a `b` because it saves 4 *entire*
  // bytes (!), since `main` will usually be a t32 function and thus usually
  // requires a linker shim to call.
  "ldr r0, =main",
  "bx r0",

  // TODO: should we soft reset or something if `main` returns?
}}

#[cfg(not(feature = "robust_irq_handler"))]
global_a32_fn! {_asm_runtime_irq_handler [iwram=true] {
  // handle MMIO interrupt system
  "ldr r0, [r12, #-8]        /* read IE_IF */",
  "and r0, r0, r0, LSR #16   /* r0 = IE & IF */",
  "strh r0, [r12, #-6]       /* write IF */",

  // Now the interrupt bits are in r0 as a `u16`

  // handle BIOS interrupt system
  "sub r2, r12, #(0x208+8)   /* BIOS_IF address */",
  "ldrh r1, [r2]             /* read the `has_occurred` flags */",
  "orr r1, r1, r0            /* activate the new bits, if any */",
  "strh r1, [r2]             /* update the value */",

  // Get the user handler fn pointer, call it if non-null.
  "ldr r1, =USER_IRQ_HANDLER",
  "ldr r1, [r1]",
  when!("r1" != "#0" [label_id=9] {
    with_pushed_registers!("{{r0, lr}}", {
      a32_fake_blx!(reg="r1", label_id=1),
    }),
  }),

  // return to the BIOS
  "bx lr",
}}

#[cfg(feature = "robust_irq_handler")]
global_a32_fn! {_asm_runtime_irq_handler [iwram=true] {
  // Suppress IME while this is running. If the user wants to allow for
  // interrupts *during* other interrupts they can enable IME in their handler.
  "add r12, r0, #0x208",
  "mov r3, #0",
  while_swapped! { ptr="r12", val="r3" {
    // handle MMIO interrupt system
    "ldr r0, [r12, #-8]        /* read IE_IF */",
    "and r0, r0, r0, LSR #16   /* r0 = IE & IF */",
    "strh r0, [r12, #-6]       /* write IF */",

    // Now the interrupt bits are in r0 as a `u16`

    // handle BIOS interrupt system
    "sub r2, r12, #(0x208+8)   /* BIOS_IF address */",
    "ldrh r1, [r2]             /* read the `has_occurred` flags */",
    "orr r1, r1, r0            /* activate the new bits, if any */",
    "strh r1, [r2]             /* update the value */",

    // Get the user handler fn pointer, call it if non-null.
    "ldr r1, =USER_IRQ_HANDLER",
    "ldr r1, [r1]",
    when!("r1" != "#0" [label_id=9] {
      with_spsr_held_in!("r2", {
        // We have to preserve:
        // * r2: spsr
        // * r3: old IME value
        // * r12: IME address
        // * lr: our handler return address
        with_pushed_registers!("{{r2, r3, r12, lr}}", {
          // Note(Lokathor): LLVM won't ever leave the stack alignment as less
          // than 8 so we skip trying to align it to 8 by hand.
          set_cpu_control!(System, irq_masked: false, fiq_masked: false),
          a32_fake_blx!(reg="r1", label_id=1),
          set_cpu_control!(Supervisor, irq_masked: true, fiq_masked: false),
        }),
      })
    }),
  }},

  // return to the BIOS
  "bx lr",
}}

/// The user-provided interrupt request handler function.
#[no_mangle]
#[cfg(feature = "on_gba")]
pub static USER_IRQ_HANDLER: GbaCell<Option<unsafe extern "C" fn(u16)>> =
  GbaCell::new(None);
