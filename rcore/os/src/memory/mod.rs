pub mod config;
pub mod heap;
pub mod address;
pub mod frame;
pub mod range;
pub mod allocator;

/// 一个缩写，模块中一些函数会使用src/memory/frame/allocator.rs:pub fn alloc(&mut self) -> MemoryResult<FrameTracker>
pub type MemoryResult<T> = Result<T, &'static str>;

/// 初始化内存相关的子模块
///
/// - [`heap::init`]
pub fn init() {
    heap::init();
    // 允许内核读写用户态内存
    unsafe { riscv::register::sstatus::set_sum() };

    println!("mod memory initialized");
}