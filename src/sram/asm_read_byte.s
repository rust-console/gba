@
@ char SramReadByte(const char* offset);
@
    .thumb
    .global SramReadByte
    .thumb_func
    .align 2
SramReadByte:
    ldr r1, =SramReadByteInner
    bx r1

    .pool
    .section .data

    .thumb
    .thumb_func
    .align 2
SramReadByteInner:
    ldrb r0, [r0]
    mov pc, lr

    .section .text
