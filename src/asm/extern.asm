/* This file exists for nice names in rust with extern "C" for the symbols */

.section .rodata

.global MEMORY_START
MEMORY_START: .dword _memory_start

.global TEXT_START
.global TEXT_END
TEXT_START: .dword _text_start
TEXT_END: .dword _text_end

.global RODATA_START
.global RODATA_END
RODATA_START: .dword _rodata_start
RODATA_END: .dword _rodata_end

.global DATA_START
.global DATA_END
DATA_START: .dword _data_start
DATA_END: .dword _data_end

.global BSS_START
.global BSS_END
BSS_START: .dword _bss_start
BSS_END: .dword _bss_end

.global KERNEL_STACK_START
.global KERNEL_STACK_END
KERNEL_STACK_START: .dword _stack_start
KERNEL_STACK_END: .dword _stack_end

.global HEAP_START
.global HEAP_SIZE
HEAP_START: .dword _heap_start
HEAP_SIZE: .dword _heap_size

.global MEMORY_END
MEMORY_END: .dword _memory_end
