// use crate::memory::{address::{PhysicalAddress, PhysicalPageNumber}, frame::FRAME_ALLOCATOR};
use super::*;

/// 分配出的物理页
///
/// 页框的tracker
pub struct FrameTracker(pub(super) PhysicalPageNumber);

impl FrameTracker {
    /// 帧的物理地址
    pub fn address(&self) -> PhysicalAddress {
        self.0.into()
    }
    /// 帧的物理页号
    pub fn page_number(&self) -> PhysicalPageNumber {
        self.0
    }
}

/// 帧在释放时会放回 [`static@FRAME_ALLOCATOR`] 的空闲链表中
impl Drop for FrameTracker {
    fn drop(&mut self) {
        FRAME_ALLOCATOR.lock().dealloc(self);
    }
}


// src/memory/mapping/mapping.rs:127:21
//     |
// 127 |                     (*frame).copy_from_slice(&page_data);
/// `FrameTracker` 可以 deref 得到对应的 `[u8; PAGE_SIZE]`
impl core::ops::Deref for FrameTracker {
    type Target = [u8; PAGE_SIZE];
    fn deref(&self) -> &Self::Target {
        self.page_number().deref_kernel()
    }
}

/// `FrameTracker` 可以 deref 得到对应的 `[u8; PAGE_SIZE]`
impl core::ops::DerefMut for FrameTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.page_number().deref_kernel()
    }
}
