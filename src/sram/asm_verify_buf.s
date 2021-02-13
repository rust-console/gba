@
@ bool SramVerifyBuf(const char* buf1, const char* buf2, int count);
@
    .thumb
    .global SramVerifyBuf
    .thumb_func
    .align 2
SramVerifyBuf:
    push {r4-r5, lr}
    movs r5, r0     @ set up r5 to be r0, so we can use it immediately for the return result
    movs r0, #0     @ set up r0 so the default return result is false
    ldr r4, =SramVerifyBufInner
    bx r4          @ jump to the part in SRAM

    .pool
    .section .data

    .thumb
    .thumb_func
    .align 2
SramVerifyBufInner:
    @ At this point, buf1 is actually in r5, so r0 can be used as a status return
    ldrb r3, [r5,r2]
    ldrb r4, [r1,r2]
    cmp r3, r4
    bne 0f
    subs r2, #1
    bpl SramVerifyBufInner

    @ Returns from the function successfully
    movs r0, #1
0:  @ Jumps to here return the function unsuccessfully, because r0 contains 0 at this point
    pop {r4-r5, pc}

    .section .text
