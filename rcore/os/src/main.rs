#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

use crate::selfcheck::*;

#[macro_use]
mod console;
mod interrupt;
mod panic;
mod sbi;
mod memory;
mod selfcheck;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    interrupt::init();
    check_int();

    memory::init();
    check_heap();
    check_physical_page();

    let remap = memory::mapping::MemorySet::new_kernel().unwrap();
    remap.activate();

    println!("kernel remapped");

    println!("\nPleased to meet you, hope you guess my- name.");
    loop {}
}
