/// 表示一段连续的页面
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Range<T: From<usize> + Into<usize> + Copy> {
    pub start: T,
    pub end: T,
}

/// 创建一个区间，其实不知道和core里的Range同名是不是好方法
impl<T: From<usize> + Into<usize> + Copy, U: Into<T>> From<core::ops::Range<U>> for Range<T> {
    fn from(range: core::ops::Range<U>) -> Self {
        Self {
            start: range.start.into(),
            end: range.end.into(),
        }
    }
}

impl<T: From<usize> + Into<usize> + Copy> Range<T> {
    /// 迭代区间中的所有页
    pub fn iter(&self) -> impl Iterator<Item = T> {
        (self.start.into()..self.end.into()).map(T::from)
    }
    pub fn len(&self) -> usize {
        self.end.into() - self.start.into()
    }

    /// 支持物理 / 虚拟页面区间互相转换
    pub fn into<U: From<usize> + Into<usize> + Copy + From<T>>(self) -> Range<U> {
        Range::<U> {
            start: U::from(self.start),
            end: U::from(self.end),
        }
    }
}
