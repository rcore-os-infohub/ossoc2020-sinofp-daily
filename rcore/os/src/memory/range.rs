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
    pub fn len(&self) -> usize {
        self.end.into() - self.start.into()
    }
}
