.section .text.boot
.set noreorder
.globl _start

_start:
    li   $sp, 0x88000000
    jal kmain
    nop

loop:
    j loop
    nop
