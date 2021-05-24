@
@ char WramReadByte(const char* offset);
@
@ A routine that reads a byte from a given memory offset.
@
    .thumb
    .global WramReadByte
    .thumb_func
    .align 2
WramReadByte:
    ldr r1, =WramReadByteInner
    bx r1

    .section .data

    .thumb
    .thumb_func
    .align 2
WramReadByteInner:
    ldrb r0, [r0]
    mov pc, lr

    .section .text

@
@ bool WramVerifyBuf(const char* buf1, const char* buf2, int count);
@
@ A routine that compares two memory offsets.
@
    .thumb
    .global WramVerifyBuf
    .thumb_func
    .align 2
WramVerifyBuf:
    push {{r4-r5, lr}}
    movs r5, r0     @ set up r5 to be r0, so we can use it immediately for the return result
    movs r0, #0     @ set up r0 so the default return result is false
    ldr r4, =WramVerifyBufInner
    bx r4          @ jump to the part in WRAM

    .section .data

    .thumb
    .thumb_func
    .align 2
WramVerifyBufInner:
    @ At this point, buf1 is actually in r5, so r0 can be used as a status return
    ldrb r3, [r5,r2]
    ldrb r4, [r1,r2]
    cmp r3, r4
    bne 0f
    subs r2, #1
    bpl WramVerifyBufInner

    @ Returns from the function successfully
    movs r0, #1
0:  @ Jumps to here return the function unsuccessfully, because r0 contains 0 at this point
    pop {{r4-r5, pc}}

    .section .text

@
@ void WramXferBuf(const char* source, char* dest, int count);
@
@ A routine that copies one buffer into another.
@
    .thumb
    .global WramXferBuf
    .thumb_func
    .align 2
WramXferBuf:
    ldr r3, =WramXferBufInner
    bx r3

    .pool
    .section .data

    .thumb
    .thumb_func
    .align 2
WramXferBufInner:
    subs r2, #1
    ldrb r3, [r0,r2]
    strb r3, [r1,r2]
    bne WramXferBufInner
    mov pc, lr

    .pool
    .section .text
