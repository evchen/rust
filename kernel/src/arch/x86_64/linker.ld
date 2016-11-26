ENTRY(start)

SECTIONS {
	. = 1M;

	.boot :
	{
		/*ensure that the multiboot header is at the beginning for GUB to find*/
		KEEP(*(.multiboot_header))
	}

	.text :
	{
		*(.text)
	}
}
