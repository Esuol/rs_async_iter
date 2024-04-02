use crate::IntoIterator;
use crate::Iterator;

// 目的是为了提供一种从迭代器中获取元素并添加到集合中的方式。具体的实现将在实现 Extend trait 的类型中提供。
pub trait Extend<A> {
    async fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = A>;
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T> Extend<T> for std::vec::Vec<T> {
    async fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let mut iter = iter.into_iter().await;
        self.reserve(iter.size_hint().1.unwrap_or_default());
        while let Some(item) = iter.next().await {
            self.push(item);
        }
    }
}
