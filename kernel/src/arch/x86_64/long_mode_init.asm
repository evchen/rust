global long_mode_start
extern rust_main
section .text
bits 64
long_mode_start:


  call rust_main

.os_returned:
  mov rax, 0x4f724f204f534f4f
  mov [0xb8000], rax
  mov rax, 0x4f724f754f744f65
  mov [0xb8008], rax
  mov rax, 0x4f214f644f654f6e
  mov [0xb8010], rax

  mov rax, 0x2f592f412f4b2f4f
  mov qword [0xb8018],rax


  hlt
