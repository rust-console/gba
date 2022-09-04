#![no_std]
#![feature(asm_sym)]
#![feature(asm_const)]
#![feature(isa_attribute)]
#![feature(naked_functions)]

use gba_cell::GbaCell;

pub mod bios;
pub mod gba_cell;

#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".text.gba_rom_header"]
unsafe extern "C" fn __start() -> ! {
  core::arch::asm!(
    "
    b 1f
    .space 0xE0
    1: /* post header */
    mov r12, #{mmio_base}
    add r0, r12, #{waitcnt_offset}
    ldr r1, ={waitcnt_setting}
    strh r1, [r0]

    /* iwram copy */
    ldr r4, =__iwram_word_copy_count
    cmp r4, #0
    beq 1f
    ldr r0, =__iwram_start
    add r3, r12, #{dma3_offset}
    ldr r2, =__iwram_position_in_rom
    str r2, [r3] /* source */
    str r0, [r3, #4] /* destination */
    strh r4, [r3, #8] /* word count */
    mov r5, #{dma3_setting}
    strh r5, [r3, #10] /* set control bits */
    1: /* post iwram copy */

    /* bss zero */
    ldr r4, =__bss_word_clear_count
    cmp r4, #0
    beq 1f
    ldr r0, =__bss_start
    mov r2, #0
    2:
    str r2, [r0], #4
    subs r4, r4, #1
    bne 2b
    1: /* post bss zero */

    /* assign the runtime irq handler */
    ldr r1, ={rt0_irq_handler}
    str r1, [r12, #-4]

    /* call to rust main */
    mov lr, #{rom_base}
    ldr r0, =main
    bx r0
    ",
    mmio_base = const 0x0400_0000,
    waitcnt_offset = const 0x204,
    waitcnt_setting = const 0x4317 /*sram8,r0:3.1,r1:4.2,r2:8.2,no_phi,prefetch*/,
    rom_base = const 0x0800_0000,
    dma3_offset = const 0xD4,
    dma3_setting = const 0x8400 /*u32,enabled*/,
    rt0_irq_handler = sym rt0_irq_handler,
    options(noreturn)
  )
}

#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.rt0.irq_handler"]
unsafe extern "C" fn rt0_irq_handler() {
  core::arch::asm!(
    "
    /*handle_irq_with_interrupts_off*/
    add r12, r0, #{ime_offset} @r12=&IME
    mov r3, #0
    swp r3, r3, [r12]   @IME swap off
    /* Still Important
    * r12, IME
    * r3, ime_previous
    */

    /*read_update_hardware_flags*/
    ldr r0, [r12, #-8]      @r0=IE_IF
    and r0, r0, r0, LSR #16 @r0=IE&IF
    strh r0, [r12, #-6]     @IF=r0
    /* Still Important
    * r12, IME
    * r3, ime_previous
    * r0, irq_flags
    */

    /*read_update_bios_flags*/
    sub  r2, r12, #(0x208+8) @r2=&BIOS_IW
    ldrh r1, [r2]            @r1=BIOS_IW
    orr  r1, r1, r0          @r1=r1|r0
    strh r1, [r2]            @BIOS_IW=r0
    /* Still Important
    * r12, IME
    * r3, ime_previous
    * r0, irq_flags
    */

    /*get_rust_fn_ptr*/
    ldr r1, ={RUST_IRQ_HANDLER}
    ldr r1, [r1]       @r1==RUST_IRQ_HANDLER
    cmp r1, #0         @if r1==0
    beq 9f             @then skip
    /* Still Important
    * r12, IME
    * r3, ime_previous
    * r1, rust_irq_fn
    * r0, irq_flags
    */

    /*call_rust_fn_in_sys_mode*/
    mrs r2, SPSR      @save SPSR
    push {{r0, r2}}     @push SPSR (SVC)

    mov r2, #{sys_no_mask} @ SYS mode + no masking
    msr CPSR_cf, r2   @set SYS mode

    /* We need to push an even number of registers here. We also need to save,
    at minimum, r3 (ime_previous) and lr (return_address). We could also save
    r12 and any junk register, but that costs +2 cycles before *and* after the
    call, and just rebuilding the r12 value after is only 2 cycles.
    */
    push {{r3, lr}} @push regs (SYS)

    adr lr, 1f
    bx r1
    1:

    pop {{r3, lr}} @pop regs (SYS)

    mov r2, #{svc_irq_masked} @ SVC mode + IRQ masked
    msr CPSR_cf, r2   @set SVC mode

    pop {{r0, r2}}      @pop SPSR (SVC)
    msr SPSR, r2        @restore SPSR
    /* Still Important
    * r3, ime_previous
    */

    /*end_of_rt0*/
    9:
    mov r12, #{mmio_base}
    add r12, r12, #{ime_offset}
    swp r3, r3, [r12]  @IME swap previous
    bx lr
    ",
    mmio_base = const 0x0400_0000,
    ime_offset = const 0x208,
    RUST_IRQ_HANDLER = sym RUST_IRQ_HANDLER,
    sys_no_mask = const 0b00011111,
    svc_irq_masked = const 0b10010010,
    options(noreturn)
  )
}

/// A function you want called during an interrupt.
///
/// This wrapper type is required to make the function pointer be aligned to 4
/// (normally function pointers are align 1).
#[derive(Clone, Copy)]
pub struct IrqFn(pub Option<unsafe extern "C" fn(u16)>);

/// Should store an `unsafe extern "C" fn(u16)`
pub static RUST_IRQ_HANDLER: GbaCell<IrqFn> = GbaCell::new(IrqFn(None));
