//! Process management syscalls
use crate::{
    config::{MAX_SYSCALL_NUM},
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, current_user_token,
        syscall_times_add, time_info, syscall_info, mmap, munmap
    },
    timer::{get_time_us},
};
use crate::mm::get_physical;

/// exit syscall
const SYSCALL_EXIT: usize = 93;
/// yield syscall
const SYSCALL_YIELD: usize = 124;
/// gettime syscall
const SYSCALL_GETTIMEOFDAY: usize = 169;
/// taskinfo syscall
const SYSCALL_TASK_INFO: usize = 410;
/// mmap syscall
const SYSCALL_MMAP: usize = 222;
/// mummap syscall
const SYSCALL_MUNMAP: usize = 215;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    syscall_times_add(SYSCALL_EXIT);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    syscall_times_add(SYSCALL_YIELD);
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    syscall_times_add(SYSCALL_GETTIMEOFDAY);
    let us = get_time_us();
    //println!("wwwwwwwwwwwww{:?}", us);
    let physical_address = get_physical(current_user_token(), _ts as usize);
    let ptr = physical_address as *mut TimeVal;
    unsafe {
        *ptr = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        }
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info"); 
    syscall_times_add(SYSCALL_TASK_INFO);
    let syscall_tm = syscall_info();
    let tm = time_info();
    let physical_address = get_physical(current_user_token(), _ti as usize);
    let ptr = physical_address as *mut TaskInfo;
    unsafe {
        *ptr = TaskInfo {
            status : TaskStatus::Running,
            syscall_times : syscall_tm,
            time : tm / 1000 as usize,
        };
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    syscall_times_add(SYSCALL_MMAP);
    mmap(_start, _len, _port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    syscall_times_add(SYSCALL_MUNMAP);
    munmap(_start, _len)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
