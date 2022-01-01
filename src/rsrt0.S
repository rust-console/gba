.arm
.global __start

__start:
    b .Linit

    @ this is replaced with correct header info by `gbafix`
    .space 188

.Linit:
    @ Set address of user IRQ handler
    @ldr r0, =MainIrqHandler
    @mov r1, #0x04000000
    @str r0, [r1, #-4]

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
    strh r1, [r0]

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

/*
    .arm
    .global MainIrqHandler
    .align 4, 0
MainIrqHandler:
    @ Load base I/O register address
    mov r2, #0x04000000
    add r2, r2, #0x200

    @ Save IRQ stack pointer and IME
    mrs r0, spsr
    ldrh r1, [r2, #8]
    stmdb sp!, {r0-r2,lr}

    @ Disable all interrupts by writing to IME
    @ r2 (0x4000200) can be used as we only care about bit 0 being unset
    strh r2, [r2, #8]

    @ Acknowledge all received interrupts that were enabled in IE
    ldr r3, [r2, #0]
    and r0, r3, r3, lsr #16
    strh r0, [r2, #2]

    @ Switch from IRQ mode to system mode
    @ cpsr_c = 0b000_10010u8 | 0b000_01101u8
    mrs r2, cpsr
    orr r2, r2, #0xD
    msr cpsr_c, r2

    @ Jump to user specified IRQ handler
    ldr r2, =__IRQ_HANDLER
    ldr r1, [r2]
    stmdb sp!, {lr}
    adr lr, .Lreturn
    bx r1
.Lreturn:
    ldmia sp!, {lr}

    @ Switch from ??? mode to IRQ mode, disable IRQ
    @ cpsr_c = ( !0b000_01101u8 & cpsr_c ) | 0b100_10010u8
    mrs r2, cpsr
    bic r2, r2, #0xD
    orr r2, r2, #0x92
    msr cpsr_c, r2

    @ Restore IRQ stack pointer and IME
    ldmia sp!, {r0-r2,lr}
    strh r1, [r2, #8]
    msr spsr_cf, r0

    @ Return to BIOS IRQ handler
    bx lr
    .pool
*/
