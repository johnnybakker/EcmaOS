.section .text._start

.global	_start

// Entry point for all cores
_start:
	mrs 	x0, MPIDR_EL1 				// Get the core number from cpu register
	ldr		x1, {0}      				// Core ID mask
	and 	x0, x0, x1					// And the value with 0b11
	ldr		x1, {1}      				// Boot Core ID
	cmp		x0, x1						// Compare the core number with input core number
	b.eq 	2f 							// If the core number does match than jump over wfe loop
1:  wfe									// Wait for event
    b       1b							// Branch back to label 1
2:	ldr     x0, =__bss_start     		// Load __bss_start address provided by the linker into register
    ldr     x1, =__bss_end      		// Load __bss_end address provided by the linker into register
3:  cmp		x0, x1						// Compare the two adresses
	b.eq	4f							// If they are the same then branch to stack initialization
	stp		xzr, xzr, [x0], #16			// Store zero at addres of x and offset it with 16 bits 2 bytes
	b		3b							// Brach back to adress comparison
4:	ldr     x1, =__stack_start			// Load stack address provided by the linker script
    mov     sp, x1						// Set stack address
	b		{2}