mod extend;
mod from_iterator;
mod into_iterator;
mod iter;
mod lending_iter;

pub use crate::iter::{Iterator, Map};
pub use from_iterator::FromIterator;
pub use into_iterator::IntoIterator;
pub use lending_iter::LendingIterator;

pub mod prelude {
    pub use crate::extend::Extend;
    pub use crate::from_iterator::FromIterator;
    pub use crate::into_iterator::IntoIterator;
}

#[cfg(feature = "alloc")]
extern crate alloc as std;
