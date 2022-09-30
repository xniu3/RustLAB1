example::main:
sub     rsp, 184
lea     rax, [rip + .L__unnamed_4]
mov     qword ptr [rsp + 72], rax
mov     qword ptr [rsp + 80], 1
lea     rax, [rip + .L__unnamed_5]
mov     qword ptr [rsp + 88], rax
mov     qword ptr [rsp + 96], 1
mov     rcx, qword ptr [rsp + 72]
mov     rax, qword ptr [rsp + 80]
mov     qword ptr [rsp + 40], rcx
mov     qword ptr [rsp + 48], rax
mov     rcx, qword ptr [rsp + 88]
mov     rax, qword ptr [rsp + 96]
mov     qword ptr [rsp + 56], rcx
mov     qword ptr [rsp + 64], rax
mov     rdi, qword ptr [rsp + 40]
mov     rsi, qword ptr [rsp + 48]
mov     rdx, qword ptr [rsp + 56]
mov     rcx, qword ptr [rsp + 64]
call    example::compare
lea     rdi, [rsp + 40]
call    qword ptr [rip + core::fmt::ArgumentV1::new_display@GOTPCREL]
mov     qword ptr [rsp + 24], rax
mov     qword ptr [rsp + 32], rdx
lea     rdi, [rsp + 56]
call    qword ptr [rip + core::fmt::ArgumentV1::new_display@GOTPCREL]
mov     qword ptr [rsp + 8], rax
mov     qword ptr [rsp + 16], rdx
mov     rax, qword ptr [rsp + 16]
mov     rcx, qword ptr [rsp + 8]
mov     rdx, qword ptr [rsp + 32]
mov     rsi, qword ptr [rsp + 24]
mov     qword ptr [rsp + 152], rsi
mov     qword ptr [rsp + 160], rdx
mov     qword ptr [rsp + 168], rcx
mov     qword ptr [rsp + 176], rax
lea     rcx, [rsp + 152]
lea     rdi, [rsp + 104]
lea     rsi, [rip + .L__unnamed_6]
mov     edx, 3
mov     r8d, 2
call    core::fmt::Arguments::new_v1
lea     rdi, [rsp + 104]
call    qword ptr [rip + std::io::stdio::_print@GOTPCREL]
add     rsp, 184
ret