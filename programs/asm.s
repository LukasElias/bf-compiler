.global _start

_start:

	// exit
	mov $60, %rax
	mov $0, %rdi
	syscall
