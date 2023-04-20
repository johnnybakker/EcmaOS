ENTRY(_start);

SECTIONS
{
    . = 0x80000;

	__stack_start = .;

    .text : {
        KEEP(*(.text._start))
        *(.text._start_arguments)
        *(.text._start_rust)
        *(.text*)
    }

    .rodata : ALIGN(8) { *(.rodata*) }

    .data : { *(.data*) }

	__bss_start = .;
    .bss : ALIGN(8) { *(.bss*); }
	__bss_end = .;

	.got : { *(.got*) }

	/DISCARD/ : { *(.ARM.exidx .ARM.exidx.*); }
}