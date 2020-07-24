pub fn check_int() {
    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    };
    println!("Interrupt OK")
}

pub fn check_heap() {
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
    println!("Heap OK");
}

pub fn check_physical_page() {
    use crate::memory;

    // 注意这里的 KERNEL_END_ADDRESS 为 ref 类型，需要加 *
    println!("KERNEL_END_ADDRESS: {}", *memory::config::KERNEL_END_ADDRESS);

    // 物理页分配
    for _ in 0..2 {
        let frame_0 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        let frame_1 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        println!("{} and {}", frame_0.address(), frame_1.address());
    }
    println!("Physical page OK");
}