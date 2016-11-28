global long_mode_start
extern rust_main
section .text
bits 64
long_mode_start:


  call rust_main




  hlt
