
.arm @ must close with a matching .thumb at the end of the file

@
@ HEADER AND INITIALIZATION CODE
@ 
.section .text.gba_rom_header
  .global __start
  __start:
    b asm_init
    @ Blank all the way out to the end of the multi-boot header.
    @ This prevents mGBA from mis-detecting the ROM type.
    .space 0xE0

  asm_init:
    @ Sets WAITCNT to the safe default for most GBA game carts.
    @ See https://problemkaputt.de/gbatek.htm#gbasystemcontrol
    ldr r0, =0x04000204
    ldr r1, =0x4317
    strh r1, [r0]

  copy_iwram:
    ldr r0, =__iwram_start
    ldr r1, =__iwram_end
    ldr r2, =__iwram_position_in_rom
    sub r3, r1, r0      @ r3 = __iwram_end - __iwram_start (bytes)
    lsrs r3, r3, #2     @ r3 /= 4 (bytes to words)
    beq 1f              @ if r3 == 0, branch 1f
    ldr  r4, =0x040000D4
    str  r2, [r4]       @ set source address
    str  r0, [r4, #4]   @ set destination
    strh r3, [r4, #8]   @ set the count
    mov  r5, #0x8400    @ 32-bit transfers, DMA enabled
    strh r5, [r4, #10]  @ set the config bits
    1:

  zero_bss:
    ldr r0, =__bss_start
    ldr r1, =__bss_end
    mov r4, #0          @ the "zero" value we'll be writing
    sub r3, r1, r0      @ r3 = __bss_end - __bss_start (bytes)
    lsrs r3, r3, #2     @ r3 /= 4 (bytes to words)
    1:
    beq 2f              @ if r3 == 0, branch 2f
    str r4, [r0]        @ store 0 to [r0]
    add r0, r0, #4      @ r0 = r0.u32.add(1)
    subs r3, r3, #1     @ r3 -= 1
    b 1b                @ branch 1b
    2:

  set_the_rt_handler:
    ldr r0, =rt_irq_handler
    ldr r1, =(0x04000000 - 4)
    str r0, [r1]

  call_to_rust_main:
    ldr lr, =1f
    ldr r0, =main
    bx r0
    @ `main` should never return,
    @ but putting this safety loop costs us so little we'll just do it.
    1: b 1b

  @ this label makes the disassembly look much better.
  end_of_init_code:
.previous

@
@ RUNTIME INTERRUPT HANDLER
@
.section .iwram
  .align 4
  rt_irq_handler:
    @ Assumed: r0 == 0x04000000 (set by the BIOS irq handler)

    @ acknowledge all flagged interrupts in IF
    add   r2, r0, #0x200
    ldr   r0, [r2]
    and   r0, r0, r0, lsr #16
    strh  r0, [r2, #2]
    
    @ set those bits in BIOS_IF as well
    mov   r3, #0x04000000
    ldrh  r1, [r3, #-8]
    orr   r1, r1, r0
    strh  r1, [r3, #-8]
    
    @ load the rust handler Option<fn> and check for null
    ldr   r1, =RUST_IRQ_HANDLER
    ldr   r1, [r1]
    cmp   r1, #0
    beq   .L_end_of_rt_irq

    @ if a fn is set, switch to USR mode and call it
    add   r2, r2, #8
    mov   r12, #0
    swp   r12, r12, [r2]
    mrs   r3, SPSR
    push  {r3}
    mov   r3, #0b11111
    msr   CPSR_cf, r3
    mov   r3, sp
    bic   sp, sp, #7
    push  {r2, r3, r12, lr}
    adr   lr, .L_after_the_call
    bx    r1
    .L_after_the_call:
    pop   {r2, r3, r12, lr}
    mov   sp, r3
    mov   r3, #0b10010010
    msr   CPSR_cf, r3
    pop   {r3}
    msr   SPSR, r3
    swp   r12, r12, [r2]
    
    .L_end_of_rt_irq:
    bx    lr
.previous

@
@ RUST FN PTR LOCATION: Option<extern "C" fn(IrqBits)>
@ 
.section .bss
  .align 4
  RUST_IRQ_HANDLER:
    .zero 4
.previous

.thumb
