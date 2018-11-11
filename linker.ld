ENTRY(__start)

MEMORY {
    ewram (w!x) : ORIGIN = 0x2000000, LENGTH = 256K
    iwram (w!x) : ORIGIN = 0x3000000, LENGTH = 32K
    rom (rx)    : ORIGIN = 0x8000000, LENGTH = 32M
}

SECTIONS {
    .text : {
        KEEP(crt0.o(.text));
        *(.text .text.*);
        . = ALIGN(4);
    } >rom = 0xff

    .rodata : {
        *(.rodata .rodata.*);
        . = ALIGN(4);
    } >rom = 0xff

    __data_lma = .;
    .data : {
        __data_start = ABSOLUTE(.);
        *(.data .data.*);
        . = ALIGN(4);
        __data_end = ABSOLUTE(.);
    } >iwram AT>rom = 0xff

    /* debugging sections */
    /* Stabs */
    .stab            0 : { *(.stab) }
    .stabstr         0 : { *(.stabstr) }
    .stab.excl       0 : { *(.stab.excl) }
    .stab.exclstr    0 : { *(.stab.exclstr) }
    .stab.index      0 : { *(.stab.index) }
    .stab.indexstr   0 : { *(.stab.indexstr) }
    .comment         0 : { *(.comment) }
    /* DWARF 1 */
    .debug           0 : { *(.debug) }
    .line            0 : { *(.line) }
    /* GNU DWARF 1 extensions */
    .debug_srcinfo   0 : { *(.debug_srcinfo) }
    .debug_sfnames   0 : { *(.debug_sfnames) }
    /* DWARF 1.1 and DWARF 2 */
    .debug_aranges   0 : { *(.debug_aranges) }
    .debug_pubnames  0 : { *(.debug_pubnames) }
    /* DWARF 2 */
    .debug_info      0 : { *(.debug_info) }
    .debug_abbrev    0 : { *(.debug_abbrev) }
    .debug_line      0 : { *(.debug_line) }
   	.debug_frame     0 : { *(.debug_frame) }
    .debug_str       0 : { *(.debug_str) }
    .debug_loc       0 : { *(.debug_loc) }
    .debug_macinfo   0 : { *(.debug_macinfo) }
    /* SGI/MIPS DWARF 2 extensions */
    .debug_weaknames 0 : { *(.debug_weaknames) }
    .debug_funcnames 0 : { *(.debug_funcnames) }
    .debug_typenames 0 : { *(.debug_typenames) }
    .debug_varnames  0 : { *(.debug_varnames) }

    /* discard anything not already mentioned */
    /DISCARD/ : { *(*) }
}
