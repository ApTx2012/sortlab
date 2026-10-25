//! Classic sorting algorithm implementations.

pub mod bubble;
pub mod merge;
pub mod quick;
pub mod shell;

#[cfg(feature = "parallel")]
pub mod parallel;

pub use bubble::Bubble;
pub use merge::Merge;
pub use quick::Quick;
pub use shell::Shell;

#[cfg(feature = "parallel")]
pub use parallel::ParallelMerge;
