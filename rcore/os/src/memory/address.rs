use bit_field::BitField;

use super::*;

/// 物理地址
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PhysicalAddress(pub usize); // 没办法，要impl就只能包裹起来

/// 物理页号
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PhysicalPageNumber(pub usize);

/// 虚拟地址
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VirtualAddress(pub usize);

/// 虚拟页号
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VirtualPageNumber(pub usize);

// 输出
impl core::fmt::Display for PhysicalAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}(0x{:x})", stringify!(PhysicalAddress), self.0)
    }
}

impl core::fmt::Display for VirtualAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}(0x{:x})", stringify!(VirtualAdress), self.0)
    }
}

impl PhysicalAddress {
    /// 从物理地址经过线性映射取得 &mut 引用
    pub fn deref_kernel<T>(self) -> &'static mut T {
        VirtualAddress::from(self).deref()
    }
    /// 取得页内偏移
    pub fn page_offset(&self) -> usize {
        self.0 % PAGE_SIZE
    }
}

impl PhysicalPageNumber {
    /// 从物理地址经过线性映射取得页面
    pub fn deref_kernel(self) -> &'static mut [u8; PAGE_SIZE] {
        PhysicalAddress::from(self).deref_kernel()
    }
}

macro_rules! floor_and_ceil {
    ($some_addr_type: tt) => {
        pub const fn floor(addr: $some_addr_type) -> Self {
            Self(addr.0 / PAGE_SIZE)
        }
        pub const fn ceil(addr: $some_addr_type) -> Self {
            Self(addr.0 / PAGE_SIZE + (addr.0 % PAGE_SIZE != 0) as usize)
        }
    }
}

impl PhysicalPageNumber {
    floor_and_ceil!(PhysicalAddress);
}

impl VirtualPageNumber {
    floor_and_ceil!(VirtualAddress);
}

impl VirtualAddress {
    pub fn deref<T>(self) -> &'static mut T {
        unsafe { &mut *(self.0 as *mut T) }
    }
}

macro_rules! from_and_to_usize {
    ($some_addr_type: tt) => {
        impl From<usize> for $some_addr_type {
            fn from(u: usize) -> Self {
                Self(u)
            }
        }

        impl From<$some_addr_type> for usize {
            fn from(p: $some_addr_type) -> Self {
                p.0
            }
        }
    }
}

from_and_to_usize!(VirtualAddress);
from_and_to_usize!(PhysicalPageNumber);
from_and_to_usize!(VirtualPageNumber);

// 页号转物理地址
impl From<PhysicalPageNumber> for PhysicalAddress {
    fn from(pg: PhysicalPageNumber) -> Self {
        Self(pg.0 * PAGE_SIZE)
    }
}

impl From<VirtualPageNumber> for VirtualAddress {
    fn from(pg: VirtualPageNumber) -> Self {
        Self(pg.0 * PAGE_SIZE)
    }
}

/// 虚拟地址转物理地址
impl From<VirtualAddress> for PhysicalAddress {
    fn from(vaddr: VirtualAddress) -> Self {
        Self(vaddr.0 - KERNEL_MAP_OFFSET)
    }
}

/// 虚实地址之间的线性映射
impl From<PhysicalAddress> for VirtualAddress {
    fn from(pa: PhysicalAddress) -> Self {
        Self(pa.0 + KERNEL_MAP_OFFSET)
    }
}

/// 虚实页号之间的线性映射
impl From<VirtualPageNumber> for PhysicalPageNumber {
    fn from(vpn: VirtualPageNumber) -> Self {
        Self(vpn.0 - KERNEL_MAP_OFFSET / PAGE_SIZE)
    }
}


// map(|offset| FrameTracker(self.start_ppn + offset))
impl core::ops::Add<usize> for PhysicalPageNumber {
    type Output = Self;
    fn add(self, other: usize) -> Self {
        Self(self.0 + other)
    }
}

impl VirtualPageNumber {
    /// 得到一、二、三级页号
    pub fn levels(self) -> [usize; 3] {
        [
            self.0.get_bits(18..27),
            self.0.get_bits(9..18),
            self.0.get_bits(0..9),
        ]
    }
}