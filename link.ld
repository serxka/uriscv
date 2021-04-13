OUTPUT_ARCH("riscv")
ENTRY(_start)

MEMORY
{
	ram ( wxa ) : ORIGIN = 0x80000000, LENGTH = 128M
}

SECTIONS
{
	_memory_start = ORIGIN(ram);

	.text :
	{
		_text_start = .;
		*(.text.boot)
		*(.text .text.*)
		_text_end = .;
	} >ram AT>ram :text

	.rodata :
	{
		_rodata_start = .;
		*(.rodata .rodata.*)
		_rodata_end = .;
	} >ram AT>ram :text 

	.data : ALIGN(4096)
	{
		_data_start = .;
		*(.sdata .sdata.*)
		*(.data .data.*)
		_data_end = .;
	} >ram AT>ram :data

	.bss : ALIGN(4096)
	{
		_bss_start = .;
		*(.sbss .sbss.*)
		*(.bss .bss.*)
		_bss_end = .;
	} >ram AT>ram :bss

	/* Stack size of 32KiB */
	_stack_start = _bss_end;
	_stack_end = _stack_start + 0x8000;
	
	/* Rest of memory for heap, user programs will run here */
	_heap_start = _stack_end;
	_heap_size = _memory_end - _heap_start;

	_memory_end = ORIGIN(ram) + LENGTH(ram);
}
