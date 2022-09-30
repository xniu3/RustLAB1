example::main:
sub     rsp, 120
lea     rax, [rip + .L__unnamed_1]
mov     qword ptr [rsp + 8], rax
mov     qword ptr [rsp + 16], 1
lea     rax, [rip + .L__unnamed_2]
mov     qword ptr [rsp + 24], rax
mov     qword ptr [rsp + 32], 1
lea     rax, [rsp + 8]
mov     qword ptr [rsp + 40], rax
lea     rax, [rip + <&T as core::fmt::Display>::fmt]
mov     qword ptr [rsp + 48], rax
lea     rcx, [rsp + 24]
mov     qword ptr [rsp + 56], rcx
mov     qword ptr [rsp + 64], rax
lea     rax, [rip + .L__unnamed_3]
mov     qword ptr [rsp + 72], rax
mov     qword ptr [rsp + 80], 3
mov     qword ptr [rsp + 88], 0
lea     rax, [rsp + 40]
mov     qword ptr [rsp + 104], rax
mov     qword ptr [rsp + 112], 2
lea     rdi, [rsp + 72]
call    qword ptr [rip + std::io::stdio::_print@GOTPCREL]
add     rsp, 120
ret