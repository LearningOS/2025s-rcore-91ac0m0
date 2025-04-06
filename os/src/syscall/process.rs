//! Process management syscalls
use core::mem::size_of;

use crate::mm::{read_from_user, write_to_user};
use crate::task::{
    change_program_brk, current_user_token, enquire_syscall, exit_current_and_run_next,
    suspend_current_and_run_next, syscall_mmap_inner, syscall_munmap_inner,
};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
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
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();

    let src = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };

    write_to_user(
        current_user_token(),
        &src as *const TimeVal as *const u8,
        _ts as *const u8,
        size_of::<TimeVal>(),
    );

    0 as isize
}

const TRACE_READ: usize = 0;
const TRACE_WRITE: usize = 1;
const TRACE_TRACE: usize = 2;

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    let id_ptr: *const u8 = id as *const u8;
    match trace_request {
        TRACE_READ => {
            let mut result: u8 = 0;
            let read_success =
                read_from_user(current_user_token(), id_ptr, &mut result as *mut u8, 1);
            if read_success < 0 {
                return -1;
            }
            result.into()
        }
        TRACE_WRITE => {
            let result = write_to_user(
                current_user_token(),
                &data as *const usize as *const u8,
                id_ptr,
                1,
            );
            if result < 0 {
                -1
            } else {
                0
            }
        }
        TRACE_TRACE => enquire_syscall(id) as isize,
        _ => -1,
    }
}
// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");

    // start 没有按页大小对齐
    if start & 0xfff != 0 {
        return -1;
    }
    //prot & !0x7 != 0 (prot 其余位必须为0)
    //prot & 0x7 = 0 (这样的内存无意义)
    if port & !0x7 != 0 || port & 0x7 == 0 {
        return -1;
    }
    syscall_mmap_inner(start, len, port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    if start == 0x10000000 {
        assert!(true);
        println!("xxxxxxxxxxx {} {}", start, len);
    }
    // start 没有按页大小对齐
    if start & 0xfff != 0 {
        return -1;
    }
    // len 没有按页大小对齐
    if len & 0xfff != 0 {
        return -1;
    }
    println!("xxxxxxxxxxx");
    syscall_munmap_inner(start, len)
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
