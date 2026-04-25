.section .text._start
.global _start

_start:
    mrs     x0, mpidr_el1
    and     x0, x0, #3
    cbz     x0, 2f
1: 
    wfe
    b       1b
2:
    mov     sp, #0x80000
    b       kmain
