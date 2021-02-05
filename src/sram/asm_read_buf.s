@
@ void SramReadBuf(const char* source, const char* dest, int blkcount, int skip);
@   source   = the source buffer to copy from
@   dest     = the destination buffer to copy into
@   blkcount = the number of 8-byte blocks to copy
@   skip     = the number of bytes to skip in the first block
@
@ Copies one buffer into another with an unrolled loop. The actual read
@ instructions are stored in WRAM, allowing it to work properly with SRAM.
@
    .thumb
    .global SramReadBuf
    .thumb_func
    .align 2
SramReadBuf:
    push {r4, lr}

    @ Calculates the offset into the inner loop to jump to.
    lsls r3, #2      @ r3 *= 4
    ldr r4, =SramReadBufInner
    adds r3, r4      @ r3 += SramReadBufInner
    bx r3

    .pool
    .section .data

    .thumb
    .thumb_func
    .align 2
SramReadBufInner:
    @ The main copy loop. Unrolled 8 times.
    ldrb r3, [r0,#0]
    strb r3, [r1,#0]
    ldrb r3, [r0,#1]
    strb r3, [r1,#1]
    ldrb r3, [r0,#2]
    strb r3, [r1,#2]
    ldrb r3, [r0,#3]
    strb r3, [r1,#3]
    ldrb r3, [r0,#4]
    strb r3, [r1,#4]
    ldrb r3, [r0,#5]
    strb r3, [r1,#5]
    ldrb r3, [r0,#6]
    strb r3, [r1,#6]
    ldrb r3, [r0,#7]
    strb r3, [r1,#7]

    @ Adjusts the loop variables, and loops again if there are still more iterations.
    adds r0, #8
    adds r1, #8
    subs r2, #1
    bne SramReadBufInner

    @ Returns from the function
    pop {r4, pc}

    .pool
    .section .text
