MEMORY {
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 128K
    RAM (xrw) : ORIGIN = 0x10000000, LENGTH = 8K
}

__checksum__ = 0 - (__stack_end__ + (reset_handler + 1) + (nmi_handler + 1) + (hard_fault_handler + 1));

ENTRY(reset_handler)

SECTIONS {
    .isr_vectors : ALIGN(4) {
        KEEP(*(.isr_vectors))
    } >FLASH

    .text : ALIGN(4) {
        KEEP(*(SORT_NONE(.init)))
        KEEP(*(SORT_NONE(.fini)))
        *(.text .text.*)
    } >FLASH

    .rodata : ALIGN(4) {
        *(.rodata .rodata.*)
    } >FLASH

    /* I have no idea what this is :) */
   	.ARM.exidx : ALIGN(4)
   	{
   	    __exidx_start = .;
       *(.ARM.exidx* .gnu.linkonce.armexidx.*)
       . = ALIGN(4);
   	    __exidx_end = .;
   	} >FLASH

    .stack : ALIGN(4) {
        __stack_start__ = .;
        . += 2K;
        . = ALIGN(4);
        __stack_end__ = .;
    } >RAM

    .data : ALIGN(4) {
        __data_start__ = .;
        __data_source_start__ = LOADADDR(.data);
        *(.data .data.*)
        . = ALIGN(4);
        __data_end__ = .;
        __data_source_end__ = LOADADDR(.data);
    } >RAM AT>FLASH

    .bss : ALIGN(4) {
        __bss_start__ = .;
        *(.bss .bss.*)
        . = ALIGN(4);
        __bss_end__ = .;
    } >RAM

    .heap : ALIGN(64) {
        __heap_start__ = .;
        . += LENGTH(RAM) - (. - ORIGIN(RAM));
        . = ALIGN(64);
        __heap_end__ = .;
    } >RAM
}
