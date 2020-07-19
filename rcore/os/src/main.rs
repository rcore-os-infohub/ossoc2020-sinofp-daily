#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[macro_use]
mod console;
mod interrupt;
mod panic;
mod sbi;
mod memory;
extern crate alloc;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    interrupt::init();

    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    };

    memory::init();

    // 动态内存分配测试
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    let v = Box::new(5);
    assert_eq!(*v, 5);
    core::mem::drop(v);

    let mut vec = Vec::new();
    for i in 0..10000 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10000);
    for (i, value) in vec.into_iter().enumerate() {
        assert_eq!(value, i);
    }
    println!("heap test passed");

    // 注意这里的 KERNEL_END_ADDRESS 为 ref 类型，需要加 *
    println!("KERNEL_END_ADDRESS: {:x}", *memory::config::KERNEL_END_ADDRESS);

    println!("\nPleased to meet you, hope you guess my- name.");
    loop {}
}
