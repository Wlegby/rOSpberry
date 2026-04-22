ENTRY(_start)

SECTIONS
{
    . = 0x80000; 

    .text : {
        KEEP(*(.text._start))
        *(.text .text.*)
    }

    . = ALIGN(4096);
    .rodata : { *(.rodata .rodata.*) }

    . = ALIGN(4096);
    .data : { *(.data .data.*) }

    . = ALIGN(4096);
    __bss_start = .;
    .bss : { *(.bss .bss.*) }
    . = ALIGN(4096);
    __bss_end = .;

    /* This throws away sections we don't want in our final binary */
    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.eh_frame_hdr)
    }
}
