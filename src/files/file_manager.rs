use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::files::FileError;
use crate::files::path::{LogicalPath, DirPolicy, ResourcePath};

/// Named mount points for the virtual file system.
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Mount {
    /// Engine-internal assets (shaders, default textures).
    Engine,
    /// Game-specific assets (models, textures, levels).
    Game,
    /// Per-user data (saves, config).
    User,
}

/// Virtual file system that resolves logical paths through named mount points.
pub struct FileManager<P: LogicalPath> {
    mount_points: HashMap<Mount, PathBuf>,
    _marker: std::marker::PhantomData<P>,
}

impl<P: LogicalPath> FileManager<P> {
    /// Creates a file manager with default mount points derived from the current directory.
    pub fn new(game_name: &str) -> Self {
        let root = std::env::current_dir().expect("Current dir missing");
        let mut mount_points = HashMap::new();

        // Define the 3 main "Roots"
        mount_points.insert(Mount::Engine, root.join("engine_assets"));
        mount_points.insert(Mount::Game, root.join("game_assets").join(game_name));
        mount_points.insert(Mount::User, root.join("user_data").join(game_name));

        let fm = FileManager {
            mount_points,
            _marker: std::marker::PhantomData,
        };

        fm
    }

    /// Resolves a logical path and filename to a physical file path.
    pub fn resolve(
        &self,
        logical: P,
        file: &str,
    ) -> Result<PathBuf, FileError> {
        let res = logical.resource_path();
        let base = self.mount_points.get(&res.mount)
            .ok_or(FileError::InvalidMount)?;

        let full = base.join(&res.relative_path).join(file);

        if !full.exists() {
            match res.policy {
                DirPolicy::Required => {
                    return Err(FileError::MissingRequired)
                }
                _ => return Err(FileError::NotFound),
            }
        }

        Ok(full)
    }
}