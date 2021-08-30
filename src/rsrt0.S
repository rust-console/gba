.arm
.global __start

__start:
    b .Linit

    @ this is replaced with correct header info by `gbafix`
    .space 188

.Linit:
    @ Set address of user IRQ handler
    ldr r0, =MainIrqHandler
    mov r1, #0x04000000
    str r0, [r1, #-4]

    @ set IRQ stack pointer
    mov r0, #0x12
    msr CPSR_c, r0
    ldr sp, =0x03007FA0

    @ set user stack pointer
    mov r0, #0x1f
    msr CPSR_c, r0
    ldr sp, =0x03007F00

    @ Sets WAITCNT to the default used by GBA games
    @
    @ See https://problemkaputt.de/gbatek.htm#gbasystemcontrol for reference.
    ldr r0, =0x04000204
    ldr r1, =0x4317
    str r1, [r0]

    @ copy .data and .text_iwram section to IWRAM
    ldr r0, =__iwram_lma     @ source address
    ldr r1, =__iwram_start   @ destination address
    ldr r2, =__iwram_end
    subs r2, r1             @ length
    @ these instructions are only executed if r2 is nonzero
    @ (i.e. don't bother copying an empty .iwram section)
    addne r2, #3
    asrne r2, #2
    addne r2, #0x04000000
    swine 0xB0000

    @ jump to user code
    ldr r0, =main
    bx r0
    @ main should be `fn() -> !`, but it doesn't hurt to guard
    1: b 1b

    .arm
    .global MainIrqHandler
    .align 4, 0
MainIrqHandler:
    mov r3, #0x04000000             @ &REG_BASE
    ldr r2, [r3, #0x200]!           @ r3 = &REG_IE, r2 = REG_IE/REG_IF
    and r2, r2, r2, lsr #16         @ irq_curr = REG_IE & REG_IF (bit of interrupt)

    @ REG_IFBIOS |= irq_curr
    ldr r1, [r3, #-0x208]
    orr r1, r1, r2
    str r1, [r3, #-0x208]

    @ Check if corresponding IRQ handler is set
    ldr r0, =__IRQ_HANDLERS
    mov r12, r2

.Lirq_loop:
    @ Loop to IRQ handler array index of bit position
    add r0, #4
    lsr r12, #1
    cmp r12, #0
    bne .Lirq_loop

.Lirq_check:
    @ Check if function pointer is null
    ldr r0, [r0, #-4]
    cmp r0, #0
    bne .Lirq_found

.Lirq_none:
    @ No IRQ handler; acknowledge interrupt and return
    strh r2, [r3, #2]    @ REG_IF=irq_curr
    bx lr

.Lirq_found:
    @ Save IE and IF to r2 (REG_IE|(irq_flag<<16))
    ldrh r2, [r3]
    lsl r1, #16
    orr r2, r2, r1

    @ Set IE to 0, acknowledge IRQ
    mov r12, r1
    strh r12, [r3]

    @ Push IE, IF, and flags
    mrs r12, spsr
    stmfd sp!, {r2, r12, lr}  @ sp_irq,{ieif, spsr, lr_irq}

    @ Disable interrupts
    str r3, [r3, #8]    @ REG_IME=0

    @ Set CPU to SYS-mode, re-enable IRQ
    mrs r2, cpsr
    bic r2, r2, #0xDF
    orr r2, r2, #0x1F
    msr cpsr, r2

    @ Enable interrupts and jump to handler (Thumb)
    stmfd sp!, {r3, lr}   @ sp_sys, {&REG_IE, lr_sys}
    mov r2, #1
    str r2, [r3, #8]    @ REG_IME=1
    adr lr, .Lpost_isr
    orr r0, r0, #1      @ Thumb mode
    bx  r0

.Lpost_isr:
    @ Disable interrupts
    ldmfd sp!, {r3, lr}   @ sp_sys, {&REG_IE, lr_sys}
    ldr r0, [r3, #8]
    str r3, [r3, #8]    @ REG_IME=0

    @ Restore CPU to IRQ-mode, disable IRQ
    mrs r2, cpsr
    bic r2, r2, #0xDF
    orr r2, r2, #0x92
    msr cpsr, r2

    @ Pop IE, IF and flags
    ldmfd sp!, {r2, r12, lr}  @ sp_irq,{ieif, spsr, lr_irq}
    msr spsr, r12

    @ Restore state and return to BIOS IRQ handler
    str r2, [r3]        @ REG_IE/REG_IF= ieif
    str r0, [r3, #8]    @ Restore REG_IME
    bx lr
