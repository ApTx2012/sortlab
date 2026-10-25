//! Prelude: `use sortlab::prelude::*;` brings in every common item.

pub use crate::algorithms::{Bubble, Merge, Quick, Shell};
pub use crate::traits::{IterAdaptor, Sorter};

#[cfg(feature = "parallel")]
pub use crate::algorithms::ParallelMerge;
