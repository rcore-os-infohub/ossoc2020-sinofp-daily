use crate::memory::config::PAGE_SIZE;

/// 物理地址
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PhysicalAddress(pub usize); // 没办法，要impl就只能包裹起来

/// 物理页号
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PhysicalPageNumber(pub usize);

// 输出
impl core::fmt::Display for PhysicalAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}(0x{:x})", stringify!(PhysicalAddress), self.0)
    }
}

impl PhysicalPageNumber {
    pub const fn floor(addr: PhysicalAddress) -> Self {
        Self(addr.0 / PAGE_SIZE)
    }
    pub const fn ceil(addr: PhysicalAddress) -> Self {
        Self(addr.0 / PAGE_SIZE + (addr.0 % PAGE_SIZE != 0) as usize)
    }
}

impl From<usize> for PhysicalPageNumber {
    fn from(u: usize) -> Self {
        Self(u)
    }
}

impl From<PhysicalPageNumber> for usize {
    fn from(p: PhysicalPageNumber) -> Self {
        p.0
    }
}

// map(|offset| FrameTracker(self.start_ppn + offset))
impl core::ops::Add<usize> for PhysicalPageNumber {
    type Output = Self;
    fn add(self, other: usize) -> Self {
        Self(self.0 + other)
    }
}

// 页号转物理地址
impl From<PhysicalPageNumber> for PhysicalAddress {
    fn from(pg: PhysicalPageNumber) -> Self {
        Self(pg.0 * PAGE_SIZE)
    }
}