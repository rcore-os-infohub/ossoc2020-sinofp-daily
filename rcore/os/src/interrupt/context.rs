use riscv::register::sstatus::Sstatus;

#[derive(Debug)]
#[repr(C)]
pub struct Context {
    pub x: [usize; 32],   // 32 个通用寄存器
    pub sstatus: Sstatus, // 具有许多状态位，控制全局中断使能等
    pub sepc: usize,      // 记录触发中断的指令的地址
}
