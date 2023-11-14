### 实验
根据指导书里给出的程序栈进行重安排，从原先的最高地址往低地址分别为真正的参数字符串，长度为所有参数的字符串长度总和加上间隔的0分隔符，其次是地址指针，长度为n+1，最后是一个表示总共有多少参数的数字，长度为1，所以真正的user_sp = sp-sum(len(arg)+1)-(n+1)-1。根据这三种不同的元素我进行了区分并分别进行元素写入。

### 问答
ELF: ch6_file0.elf: ELF 64-bit LSB executable, UCB RISC-V, version 1 (SYSV), statically linked, stripped
BIN: ch6_file0.bin: data
执行BIN: riscv64-linux-musl-objdump: ch6_file0.bin: file format not recognized

elf文件属于目标文件，可直接加载到内存中，在编译过程中得到的目标代码，文件中存储了代码段，数据段以及段表等数据，而bin文件则只有代码段和数据段的存储，需要进行链接后才能加载到内存中。