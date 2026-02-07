use std::marker::PhantomData;
use std::path::PathBuf;

/// A resource type that can be loaded from a file path.
pub trait Asset: Sized + 'static {
    /// Error type returned when loading fails.
    type Error;

    /// Loads the asset from the given file path.
    fn load(path: PathBuf) -> Result<Self, Self::Error>;
}


