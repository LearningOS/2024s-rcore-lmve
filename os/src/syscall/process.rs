//! Process management syscalls
// use core::{array::try_from_fn, intrinsics::size_of};

// use crate::{
//     config::MAX_SYSCALL_NUM, mm::translated_byte_buffer, task::{
//         change_program_brk, current_user_token, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus
//     }
// };

use core::mem::size_of;


use crate::{
    config::MAX_SYSCALL_NUM,
    mm::{translated_byte_buffer,MapPermission,VirtAddr},
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_dispatched_time, get_syscall_times, get_task_status,mmap,munmap,suspend_current_and_run_next, TaskStatus
    }
};

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
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ? 
/*
 * _ts  是指向TimeVal的指针,直接访问会导致页错误，特别是在 [`TimeVal`] 跨越两个页面时。
 */
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time"); /* log sys_get_time 跟踪系统调用以进行调试或日志记录 */
    /* 获取TimeVal结构体的大小 */
    const SIZE: usize = size_of::<TimeVal>();
    /* 尝试将用户空间的缓冲区转换为可安全访问的字节缓冲区 */
    if let Ok(rgeions) = translated_byte_buffer(current_user_token(), _ts as *const u8, SIZE){
        /* 获取当前时间的微秒级别时间戳 */
        let us: usize = crate::timer::get_time_us();
        /* 创建一个用于存储时间值的临时缓冲区 */
        let mut buffer = [0u8; SIZE];
        /* 在临时缓冲区中构建一个TimeVal结构体，填充从微秒级别时间戳转换来的时间值 */
        unsafe {
            let raw_ptr = buffer.as_mut_ptr() as usize as *mut TimeVal;
            *raw_ptr = TimeVal {
                sec: us / 1_000_000,
                usec: us % 1_000_000,
            };
        }
        copy_to_segs(rgeions, &buffer);/* 将这个值拷贝到用户空间 */
        0
    } else {
        -1
    }
}


/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    const SIZE: usize = size_of::<TaskInfo>();
    if let Ok(regions) = translated_byte_buffer(current_user_token(), _ti as *const u8, SIZE){
        let mut buffer = alloc::vec![u8;SIZE]; 
        unsafe {
            let ref_coe = (buffer.as_mut_ptr() as usize as *mut TaskInfo).as_mut().unwrap();
            ref_coe.time = crate::timer::get_time_ms() - get_dispatch_time();
            ref_coe.status = get_task_status();
            get_syscall_times(&mut ref_coe.syscall_times);
        }
        copy_to_segs(regions, &buffer);
        0
    } else {
        -1
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if _start % crate::config::PAGE_SIZE != 0 {
        return -1;
    }
    if prot !&(0x7) != 0 || port & 0x7 == 0{
        return -1;
    }
    let start_va: VirtAddr = start.into();
    let end_va:VirtAddr = (start + len).into();
    let flags = prot (as u8) << 1;
    mmap(start_va,end_va,MapPermission::from_bits(flags).unwrap() | MapPermission::U)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    if _start % crate::config::PAGE_SIZE != 0 {
        return -1;
    }
    let star_va: VirtAddr = _start.into();
    let end_va:VirtAddr = (_start + _len).into();
    munmap(star_va,end_va)
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
