use crate::Iterator;

pub trait IntoIterator {
    type Item;

    type IntoIter: Iterator<Item = Self::Item>;

    async fn into_iter(self) -> Self::IntoIter;
}

impl<I: Iterator> IntoIterator for I {
    type Item = I::Item;
    type IntoIter = I;

    async fn into_iter(self) -> Self::IntoIter {
        self
    }
}
