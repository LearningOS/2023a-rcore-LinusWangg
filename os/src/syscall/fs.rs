//! File and filesystem-related syscalls
use crate::fs::{open_file, OpenFlags, Stat, ROOT_INODE, StatMode};
use crate::mm::{translated_byte_buffer, translated_str, UserBuffer, translated_refmut};
use crate::task::{current_task, current_user_token, syscall_add};

/// read syscall
const SYSCALL_READ: usize = 63;
/// write syscall
const SYSCALL_WRITE: usize = 64;
/// syscall linkat
const SYSCALL_LINKAT: usize = 37;
/// syscall unlinkat
const SYSCALL_UNLINKAT: usize = 35;
/// syscall fstat
const SYSCALL_FSTAT: usize = 80;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    trace!("kernel:pid[{}] sys_write", current_task().unwrap().pid.0);
    syscall_add(SYSCALL_WRITE);
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        if !file.writable() {
            return -1;
        }
        let file = file.clone();
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        file.write(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -1
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    trace!("kernel:pid[{}] sys_read", current_task().unwrap().pid.0);
    syscall_add(SYSCALL_READ);
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();
        if !file.readable() {
            return -1;
        }
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        trace!("kernel: sys_read .. file.read");
        file.read(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -1
    }
}

pub fn sys_open(path: *const u8, flags: u32) -> isize {
    trace!("kernel:pid[{}] sys_open", current_task().unwrap().pid.0);
    let task = current_task().unwrap();
    let token = current_user_token();
    let path = translated_str(token, path);
    if let Some(inode) = open_file(path.as_str(), OpenFlags::from_bits(flags).unwrap()) {
        let mut inner = task.inner_exclusive_access();
        let fd = inner.alloc_fd();
        inner.fd_table[fd] = Some(inode);
        fd as isize
    } else {
        -1
    }
}

pub fn sys_close(fd: usize) -> isize {
    trace!("kernel:pid[{}] sys_close", current_task().unwrap().pid.0);
    let task = current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    inner.fd_table[fd].take();
    0
}

/// YOUR JOB: Implement fstat.
pub fn sys_fstat(_fd: usize, _st: *mut Stat) -> isize {
    trace!("kernel:pid[{}] sys_fstat", current_task().unwrap().pid.0);
    syscall_add(SYSCALL_FSTAT);
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();

    if _fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[_fd].is_none() {
        return -1;
    }
    if let Some(fd_file) = &inner.fd_table[_fd] {
        let (inode_id, dir) = fd_file.inode_id();
        let stat_mode: StatMode;
        if dir == 1 {
            stat_mode = StatMode::DIR;
        }
        else{
            stat_mode = StatMode::FILE;
        };
        let nlink = ROOT_INODE.link_count(inode_id);
        drop(inner);
        let st = translated_refmut(token, _st);

        st.dev = 0;
        st.ino = inode_id as u64;
        st.mode = stat_mode;
        st.nlink = nlink as u32;
        return 0;
    } else {
        return -1;
    }
}

/// YOUR JOB: Implement linkat.
pub fn sys_linkat(_old_name: *const u8, _new_name: *const u8) -> isize {
    trace!("kernel:pid[{}] sys_linkat", current_task().unwrap().pid.0);
    syscall_add(SYSCALL_LINKAT);
    let token = current_user_token();
    let old_name = translated_str(token, _old_name);
    let new_name = translated_str(token, _new_name);
    if let Some(inode) = ROOT_INODE.find(&old_name as &str) {
        let inode_id = inode.find_inode_id_by_inode() as u32;
        ROOT_INODE.linkat(inode_id, &new_name as &str)
    } else {
        return -1;
    }
}

/// YOUR JOB: Implement unlinkat.
pub fn sys_unlinkat(_name: *const u8) -> isize {
    trace!("kernel:pid[{}] sys_unlinkat", current_task().unwrap().pid.0);
    syscall_add(SYSCALL_UNLINKAT);
    let token = current_user_token();
    let name = translated_str(token, _name);
    ROOT_INODE.unlinkat(&name as &str)
}
