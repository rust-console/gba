#[inline]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.mem"]
pub unsafe extern "C" fn forward_copy_u8(
  dest: *mut u8, src: *const u8, byte_count: usize,
) {
  core::arch::asm! {
    "1:",
    "subs    {count}, {count}, #1",
    "ldrbge  {temp}, [{src}], #1",
    "strbge  {temp}, [{dest}], #1",
    "bgt     1b",
    temp = out(reg) _,
    count = inout(reg) byte_count => _,
    src = inout(reg) src => _,
    dest = inout(reg) dest => _,
    options(nostack)
  }
}

#[inline]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.mem"]
pub unsafe extern "C" fn forward_copy_u16(
  mut dest: *mut u16, mut src: *const u16, mut byte_count: usize,
) {
  core::arch::asm! {
    "1:",
    "subs    {count}, {count}, #2",
    "ldrhge  {temp}, [{src}], #2",
    "strhge  {temp}, [{dest}], #2",
    "bgt     1b",
    temp = out(reg) _,
    count = inout(reg) byte_count,
    src = inout(reg) src,
    dest = inout(reg) dest,
    options(nostack)
  }
  if byte_count != 0 {
    let dest = dest.cast::<u8>();
    let src = src.cast::<u8>();
    dest.write_volatile(src.read_volatile());
  }
}

#[naked]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.mem"]
pub unsafe extern "C" fn forward_copy_u32(
  dest: *mut u32, src: *const u32, byte_count: usize,
) {
  core::arch::asm! {
    bracer::when!( "r2" >=u "#32" [label_id=2] {
      bracer::with_pushed_registers!("{{r4-r9}}", {
        "1:",
        "subs   r2, r2, #32",
        "ldmge  r1!, {{r3-r9, r12}}",
        "stmge  r0!, {{r3-r9, r12}}",
        "bgt    1b",
      }),
      "bxeq   lr",
    }),

    // copy 4 words, two at a time
    "tst    r2, #0b10000",
    "ldmne  r1!, {{r3, r12}}",
    "stmne  r0!, {{r3, r12}}",
    "ldmne  r1!, {{r3, r12}}",
    "stmne  r0!, {{r3, r12}}",
    "bics   r2, r2, #0b10000",
    "bxeq   lr",

    // copy 2 and/or 1 words
    "lsls   r3, r2, #29",
    "ldmcs  r1!, {{r3, r12}}",
    "stmcs  r0!, {{r3, r12}}",
    "ldrmi  r3, [r1], #4",
    "strmi  r3, [r0], #4",
    "bics   r2, r2, #0b1100",
    "bxeq   lr",

    // copy halfword and/or byte
    "lsls   r3, r2, #31",
    "ldrhcs r3, [r1], #2",
    "strhcs r3, [r0], #2",
    "ldrbmi r3, [r1], #1",
    "strbmi r3, [r0], #1",
    "bx     lr",
    options(noreturn),
  }
}

#[inline]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.mem"]
pub unsafe extern "C" fn reverse_copy_u8(
  dest: *mut u8, src: *const u8, byte_count: usize,
) {
  core::arch::asm! {
    "1:",
    "subs    {count}, {count}, #1",
    "ldrbge  {temp}, [{src}, #-1]!",
    "strbge  {temp}, [{dest}, #-1]!",
    "bgt     1b",
    temp = out(reg) _,
    count = inout(reg) byte_count => _,
    src = inout(reg) src => _,
    dest = inout(reg) dest => _,
    options(nostack)
  }
}

#[inline]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.mem"]
pub unsafe extern "C" fn reverse_copy_u16(
  mut dest: *mut u16, mut src: *const u16, mut byte_count: usize,
) {
  core::arch::asm! {
    "1:",
    "subs    {count}, {count}, #2",
    "ldrhge  {temp}, [{src}, #-2]!",
    "strhge  {temp}, [{dest}, #-2]!",
    "bgt     1b",
    temp = out(reg) _,
    count = inout(reg) byte_count,
    src = inout(reg) src,
    dest = inout(reg) dest,
    options(nostack)
  }
  if byte_count != 0 {
    let dest = dest.cast::<u8>().sub(1);
    let src = src.cast::<u8>().sub(1);
    dest.write_volatile(src.read_volatile());
  }
}

#[naked]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.mem"]
pub unsafe extern "C" fn reverse_copy_u32(
  dest: *mut u32, src: *const u32, byte_count: usize,
) {
  core::arch::asm! {
    bracer::when!( "r2" >=u "#32" [label_id=2] {
      bracer::with_pushed_registers!("{{r4-r9}}", {
        "1:",
        "subs    r2, r2, #32",
        "ldmdbcs r1!, {{r3-r9, r12}}",
        "stmdbcs r0!, {{r3-r9, r12}}",
        "bgt     1b",
      }),
      "bxeq   lr",
    }),

    // copy 4 words, two at a time
    "tst     r2, #0b10000",
    "ldmdbne r1!, {{r3, r12}}",
    "stmdbne r0!, {{r3, r12}}",
    "ldmdbne r1!, {{r3, r12}}",
    "stmdbne r0!, {{r3, r12}}",
    "bics    r2, r2, #0b10000",
    "bxeq    lr",

    // copy 2 and/or 1 words
    "lsls    r3, r2, #29",
    "ldmdbcs r1!, {{r3, r12}}",
    "stmdbcs r0!, {{r3, r12}}",
    "ldrmi   r3, [r1, #-4]!",
    "strmi   r3, [r0, #-4]!",
    "bxeq    lr",

    // copy halfword and/or byte
    "lsls    r2, r2, #31",
    "ldrhcs  r3, [r1, #-2]!",
    "strhcs  r3, [r0, #-2]!",
    "ldrbmi  r3, [r1, #-1]!",
    "strbmi  r3, [r0, #-1]!",
    "bx      lr",
    options(noreturn),
  }
}
