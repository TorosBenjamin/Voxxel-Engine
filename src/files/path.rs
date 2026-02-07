use std::path::PathBuf;
use crate::files::file_manager::Mount;

/// How to handle a missing directory when resolving a path.
pub enum DirPolicy {
    /// Panic if the directory is missing.
    Required,
    /// Create the directory if missing.
    AutoCreate,
    /// Return an error if missing.
    Optional,
}

/// Implemented by game-defined path enums to map logical locations to mount points.
pub trait LogicalPath: Copy + Eq + std::hash::Hash {
    /// Returns the mount, policy, and relative path for this logical location.
    fn resource_path(&self) -> ResourcePath;
}

/// A resolved logical path: mount point, directory policy, and relative path.
pub struct ResourcePath {
    /// Which mount point this path belongs to.
    pub mount: Mount,
    /// How to handle a missing directory.
    pub policy: DirPolicy,
    /// Path relative to the mount point root.
    pub relative_path: PathBuf,
}