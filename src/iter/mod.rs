#[must_use = "iterators are lazy and do nothing unless consumed"]
pub trait Iterator {
    type Item;

    async fn next(&mut self) -> Option<Self::Item>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    #[must_use = "iterators do nothing unless iterated over"]
    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        Map::new(self, f)
    }
}
