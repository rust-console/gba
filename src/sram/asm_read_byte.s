@
@ char SramReadByte(const char* offset);
@   offset = the offset to read a byte from
@   return = the byte read from that offset 
@
@ Reads a byte from the target offset. The actual read instruction is stored in
@ WRAM, allowing it to work properly with SRAM reads.
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

    .pool
    .section .text
