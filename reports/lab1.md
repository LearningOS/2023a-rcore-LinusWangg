### Author: LinusWangg
## 简答作业
1. 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容 (运行 Rust 三个 bad 测例 (ch2b_bad_*.rs) ， 注意在编译时至少需要指定 LOG=ERROR 才能观察到内核的报错信息) ， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。
    [rustsbi] Implementation     : RustSBI-QEMU Version 0.2.0-alpha.2
    [rustsbi] Platform Name      : riscv-virtio,qemu
    [rustsbi] Platform SMP       : 1
    [rustsbi] Platform Memory    : 0x80000000..0x88000000
    [rustsbi] Boot HART          : 0
    [rustsbi] Device Tree Region : 0x87000000..0x87000ef2
    [rustsbi] Firmware Address   : 0x80000000
    [rustsbi] Supervisor Address : 0x80200000
    [rustsbi] pmp01: 0x00000000..0x80000000 (-wr)
    [rustsbi] pmp02: 0x80000000..0x80200000 (---)
    [rustsbi] pmp03: 0x80200000..0x88000000 (xwr)
    [rustsbi] pmp04: 0x88000000..0x00000000 (-wr)
    [kernel] Hello, world!
    [ERROR] [kernel] .bss [0x80263000, 0x8028c000)
    [kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003c4, kernel killed it.
    [kernel] IllegalInstruction in application, kernel killed it.
    [kernel] IllegalInstruction in application, kernel killed it.
    [kernel] Panicked at src/trap/mod.rs:72 Unsupported trap Exception(LoadFault), stval = 0x18!

    sbi版本：RustSBI-QEMU Version 0.2.0-alpha.2
2. 深入理解 trap.S 中两个函数 __alltraps 和 __restore 的作用，并回答如下问题:
    1. a0为trap_handler的返回值。系统中断后重新进入用户态和初始进入用户态。
    2. t2为内核栈指针地址，返回内核态时可以从t2得到内核栈。t0记录了中断发生前的操作状态。t1为进入内核态前用户程序发生中断的指令地址。
    3. csrw sscratch, t2中已经保存x2进入sscratch。x4不知道。
    4. sp指向用户程序栈，sscratch指向内核栈。
    5. sret指令：从内核态返回用户态，同时将 pc 的值设置为 sepc。（如果需要返回到 sepc 后一条指令，就需要在 sret 之前修改 sepc 的值）
    6. 与4相反.
    7. ecall

## 功能实现
在任务的mod模块封装当前任务的系统调用次数和启动时间并使用函数将当前任务的信息回传到process的sys_task_info模块，在每次系统调用时通过调用mod函数对inner属性进行修改。

## 荣誉准则
1. 在完成本次实验的过程，我没有与人交流。

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

《rCore-Tutorial-Guide 2023A 文档》
《The RISC-V Instruction Set Manual Volume II: Privileged Architecture》
《RISC-V 与中断相关的寄存器和指令》

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

## 难度评价
难度适中，刚上手比较难懂，多看几遍文档后就知道怎么做了