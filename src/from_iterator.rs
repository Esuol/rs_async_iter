use crate::IntoIterator;
use crate::Iterator;
use std::vec::Vec;

// 这个 trait 的目的是为了提供一种从迭代器中收集元素并构造出新的实例的方式。
pub trait FromIterator<A>: Sized {
    async fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self;
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T> FromIterator<T> for Vec<T> {
    async fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Vec<T> {
        let mut iter = iter.into_iter().await;
        let mut out = Vec::with_capacity(iter.size_hint().1.unwrap_or_default());
        while let Some(item) = iter.next().await {
            out.push(item);
        }
        out
    }
}
