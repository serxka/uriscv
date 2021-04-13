.option norvc                          # Disable instruction compression
.section .text.boot

.global _start
_start:
	# Any hart's that aren't bootstrapping need to wait for an IPI to start
	csrr	t0, mhartid            # Load our hart id
	bnez	t0, 3f                 # If not zero (bootstrap) go wait for a IPI
	csrw	satp, zero             # Be cautious and disable the MMU for now

# Set the global pointer, this is a relative address for the pc
.option push
.option norelax                        # Disable assembler instruction relaxing for setting the GP
	la	gp, __global_pointer$
.option pop

	# Clear the BSS to zero
	la	a0, _bss_start         # Store the start of the BSS
	la	a1, _bss_end           # Store the end of the BSS
	bgeu	a0, a1, 2f             # If the size is zero then skip zeroing
1:
	sd	zero, (a0)             # Store zero into the current pointer
	addi	a0, a0, 8              # Increment the pointer
	bltu	a0, a1, 1b             # If pointer still hasn't reach _bss_end branch back
2:
	la	sp, _stack_end         # Set the stack pointer
	li	t0, 0b11 << 11         # Set the privilege level to 3 (machine)
	csrw	mstatus, t0            # Write privilege level
	csrw	mie, zero              # Disable interupts whilst kinit is entered

	la	t0, kinit              # Load address of kinit
	csrw	mepc, t0               # Load address into machine exception program counter
	la	ra, 4f                 # Return to next '4' label
	mret                           # Return into kinit
3:
	# Setup for secondary hart's
4:
	wfi
	j	4b
