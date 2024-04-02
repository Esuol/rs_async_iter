#[must_use = "iterators are lazy and do nothing unless consumed"]
pub trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;

    async fn next(&mut self) -> Option<Self::Item<'_>>;
}
