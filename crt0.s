    .arm
__start:
    b .Linit

    @ ROM header will be filled in by gbafix
    .fill 188, 1, 0

.Linit:
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
    ldr r3, =__data_end
    sub r2, r3, r1
    beq .Lskip              @ don't try to copy an empty .data section
    add r2, #3
    mov r2, r2, asr #2      @ length (in words)
    add r2, #0x04000000     @ copy by words
    swi 0xb0000

.Lskip:
    @ jump to user code
    ldr r0, =main
    bx r0
    .pool
