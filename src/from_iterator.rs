use crate::IntoIterator;
use crate::Iterator;
use std::vec::Vec;

/// Conversion from an [`Iterator`].

pub trait FromIterator<A>: Sized {
    /// Creates a value from an iterator.
    async fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self;
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T> FromIterator<T> for Vec<T> {
    async fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Vec<T> {
        let mut iter = iter.into_iter().await;
        let mut output = Vec::with_capacity(iter.size_hint().1.unwrap_or_default());
        while let Some(item) = iter.next().await {
            output.push(item);
        }
        output
    }
}
