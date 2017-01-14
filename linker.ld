OUTPUT_FORMAT ("elf32-littlearm", "elf32-bigarm", "elf32-littlearm")

ENTRY(Reset_Handler)

SECTIONS {
    __FLASH_ORIGIN = 0x00000000;
    __FLASH_SIZE = 0x28000;
    __RAM_ORIGIN = 0x20002000;
    __RAM_SIZE = 0x2000;

    /*
     * FLASH
     */
    . = __FLASH_ORIGIN;

    .text : {
        KEEP(*(.Vectors))
        *(.text*)

        *(.rodata*)
    }

   .ARM.extab : {
        *(.ARM.extab* .gnu.linkonce.armextab.*)
    }

    __exidx_start = .;
    .ARM.exidx : {
        *(.ARM.exidx* .gnu.linkonce.armexidx.*)
    }
    __exidx_end = .;

    __FLASH_USED = . -  __FLASH_ORIGIN;
    ASSERT(__FLASH_SIZE >= __FLASH_USED, "Flash overflow")

    /* end of text/code segment */
    __etext = .;

    /*
     * RAM
     */ 
    . = __RAM_ORIGIN;

    __data_start__ = .;
    .data : {
	*(.data*)
    }
    __data_end__ = .;

    . = ALIGN(4);
    .bss : {
        *(.bss*)
    }
    . = ALIGN(4);

    . = ALIGN(3);
    __end__ = .;
    end = __end__;
    /* Everything below will not be used in the .hex */

    .heap : {
        *(.heap*)
    }
    __HeapLimit = .;

    . = ALIGN(3);
    .stack_dummy : {
        *(.stack*)
    }

    __StackTop = __RAM_ORIGIN + __RAM_SIZE ;
    __StackLimit = __StackTop - SIZEOF(.stack_dummy);
    PROVIDE(__stack = __StackTop);

    ASSERT(__StackLimit >= __HeapLimit, "region RAM overflowed with stack")
}
