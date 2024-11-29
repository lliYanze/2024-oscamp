#![allow(dead_code)]

use axerrno::LinuxError;
use axhal::arch::TrapFrame;
use axhal::mem::VirtAddr;
use axhal::paging::MappingFlags;
use axhal::trap::{register_trap_handler, PAGE_FAULT, SYSCALL};
use axtask::{current, TaskExtRef};

const SYS_EXIT: usize = 93;

#[register_trap_handler(SYSCALL)]
fn handle_syscall(tf: &TrapFrame, syscall_num: usize) -> isize {
    ax_println!("handle_syscall ...");
    let ret = match syscall_num {
        SYS_EXIT => {
            ax_println!("[SYS_EXIT]: process is exiting ..");
            axtask::exit(tf.arg0() as _)
        }
        _ => {
            ax_println!("Unimplemented syscall: {}", syscall_num);
            -LinuxError::ENOSYS.code() as _
        }
    };
    ret
}

#[register_trap_handler(PAGE_FAULT)]
fn handle_page_fault(va: VirtAddr, access_flags: MappingFlags, _is_user: bool) -> bool {
    ax_println!("handle_page_fault ...");
    if current()
        .task_ext()
        .aspace
        .lock()
        .handle_page_fault(va, access_flags)
    {
        ax_println!("OK!")
    } else {
        ax_println!("{}: segmentation fault, exit!", axtask::current().id_name());
    }

    return true;
}
