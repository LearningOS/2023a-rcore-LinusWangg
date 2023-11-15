### 实验
缺29->找到ioctl及其定义->写上去发现29不报错，应该没什么关系->缺66->找到write及其定义->写完后能输出了但是缺94->找到exitgroup及其定义补上过了。

### 问答
WNOHANG
如果没有孩子存在立即返回。
WUNTRACED
如果孩子停止(但没有通过 ptrace(2) 追踪)也返回。如果这个选项没有指定，已经停止的将通过 traced 返回其状态。
WCONTINUED (从 Linux 2.6.10 开始)
如果已经停止的孩子因为 SIGCONT 的递送而继续执行也返回。
(只对 Linux 有效的选项，见下面。)
如果 status 不是 NULL，wait() 和 waitpid() 保存状态信息在那个 int 指针指向的内存里。这个整数可以通过下面的宏(它们接受整数自身，而不指向它的指针，wait() 和 waitpid() 需要指针！)进行审视：
WIFEXITED(status)
如果孩子是正常终止则返回真，这说明孩子是调用 exit(3) 或 _exit(2)，或者由 main() 函数返回而终止。
WEXITSTATUS(status)
返回孩子的退出状态。这是 status 参数的最低 8 位值，这个值由孩子调用 exit(3) 或 _exit(2) 或者作为 main() return 语句的参数来指定。这个宏只应该在 WIFEXITED 返回真时调用。
WIFSIGNALED(status)
如果孩子进程因为一个信号而终止则返回真。
WTERMSIG(status)
返回导致孩子终止的信号个数。这个宏只应该在 WIFSIGNALED 返回真时调用。
WCOREDUMP(status)
如果孩子进程产生核心转储文件则返回真。这个宏只应该在 WIFSIGNALED 返回真时调用。这个没有在 POSIX.1-2001 里指定并且在一些 UNIX 实现(如 AIX、SunOS)里也没有提供。只在 #ifdef WCOREDUMP ... #endif 内部使用。
WIFSTOPPED(status)
如果孩子进程因为信号而停止则返真；这只有在使用了 WUNTRACED 调用或当孩子被追踪(见 ptrace(2)) 时才可能。
WSTOPSIG(status)
返回导致孩子停止的信号个数。这个宏只应该在 WIFSTOPPED 返回真时调用。
WIFCONTINUED(status)
(从 Linux 2.6.10 开始) 如果孩子进程因为 SIGCONT 信号继续执行则返回真

具体的int值可以通过位运算得到，在C++接口中可直接调用