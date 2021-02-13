@
@ void SramXferBuf(const char* source, char* dest, int count);
@
    .thumb
    .global SramXferBuf
    .thumb_func
    .align 2
SramXferBuf:
    ldr r3, =SramXferBufInner
    bx r3

    .pool
    .section .data

    .thumb
    .thumb_func
    .align 2
SramXferBufInner:
    subs r2, #1
    ldrb r3, [r0,r2]
    strb r3, [r1,r2]
    bne SramXferBufInner
    mov pc, lr

    .section .text
