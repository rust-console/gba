    .arm
__start:
    b .Linit

    @ ROM header will be filled in by gbafix
    .fill 188, 1, 0

.Linit:
    @ Set address of user IRQ handler
    ldr r0, =MainIrqHandler
    ldr r1, =0x03FFFFFC
    str r0, [r1]

    @ set IRQ stack pointer
    mov r0, #0x12
    msr CPSR_cf, r0
    ldr sp, =0x3007fa0

    @ set user stack pointer
    mov r0, #0x1f
    msr CPSR_cf, r0
    ldr sp, =0x3007f00

    @ copy .data section to IWRAM
    ldr r0, =__data_lma     @ source address
    ldr r1, =__data_start   @ destination address
    ldr r2, =__data_end
    subs r2, r1             @ length
    @ these instructions are only executed if r2 is nonzero
    @ (i.e. don't bother copying an empty .data section)
    addne r2, #3
    asrne r2, #2
    addne r2, #0x04000000
    swine 0xb0000

    @ jump to user code
    ldr r0, =main
    bx r0

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
    mov r0, #0
    strh r0, [r2, #8]

    @ Acknowledge all received interrupts that were enabled in IE
    ldr r3, [r2, #0]
    and r0, r3, r3, lsr #16
    strh r0, [r2, #2]

    @ Switch to system mode
    mrs r2, cpsr
    bic r2, r2, #0x1F
    orr r2, r2, #0x1F
    msr cpsr_cf, r2

    @ Jump to user specified IRQ handler
    ldr r2, =__IRQ_HANDLER
    ldr r1, [r2]
    stmdb sp!, {lr}
    adr lr, .Lreturn
    bx r1
.Lreturn:
    ldmia sp!, {lr}

    @ Switch to IRQ mode
    mrs r2, cpsr
    bic r2, r2, #0x1F
    orr r2, r2, #0x92
    msr cpsr_cf, r2

    @ Restore IRQ stack pointer and IME
    ldmia sp!, {r0-r2,lr}
    strh r1, [r2, #8]
    msr spsr_cf, r0

    @ Return to BIOS IRQ handler
    bx lr
    .pool
