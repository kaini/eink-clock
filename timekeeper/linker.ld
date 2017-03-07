MEMORY {
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 128K
    RAM (xrw) : ORIGIN = 0x10000000, LENGTH = 8K
}

__stack_start__ = ORIGIN(RAM);
__stack_size__ = 1K;
__stack_end__ = __stack_start__ + __stack_size__;
__stack = __stack_end__;  /* For Newlib's init code */

__bss_size__ = __bss_end__ - __bss_start__;

__data_size__ = __data_end__ - __data_start__;

__heap_start__ = __data_end__;
__heap_size__ = __heap_end__ - __heap_start__;
__heap_end__ = ORIGIN(RAM) + LENGTH(RAM);

__checksum__ = 0 - (__stack_end__ + (reset_handler + 1) + (nmi_handler + 1) + (hard_fault_handler + 1));

ENTRY(_start)

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

    .bss __stack_end__ : ALIGN(4) {
        __bss_start__ = .;
        *(.bss .bss.*)
        . = ALIGN(4);
        __bss_end__ = .;
    } >RAM

    .data __bss_end__ : ALIGN(4) {
        __data_start__ = .;
        __data_source_start__ = LOADADDR(.data);
        *(.data .data.*)
        . = ALIGN(4);
        __data_end__ = .;
    } >RAM AT>FLASH

    /* I have no idea what this is :) */
   	.ARM.exidx : ALIGN(4)
   	{
   	    __exidx_start = .;
       *(.ARM.exidx* .gnu.linkonce.armexidx.*)
       . = ALIGN(4);
   	    __exidx_end = .;
   	} >FLASH
}
