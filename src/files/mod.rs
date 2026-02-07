pub mod file_manager;
pub mod path;

/// Errors returned by the virtual file system.
pub enum FileError {
    /// The resolved path does not exist.
    NotFound,
    /// The mount point is not registered.
    InvalidMount,
    /// A required directory or file is missing.
    MissingRequired,
    /// An underlying I/O error.
    Io(std::io::Error),
}