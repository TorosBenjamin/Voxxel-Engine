use std::any::{Any, TypeId};
use std::collections::HashMap;
use crate::core::handle::Handle;
use crate::files::file_manager::FileManager;
use crate::files::path::LogicalPath;
use crate::resource::asset::{Asset};

struct AssetStorage {
    next_id: u32,
    assets: HashMap<u32, Box<dyn Any>>,
}

/// Type-erased storage for all game resources, keyed by [`Handle`].
pub struct ResourceManager<P: LogicalPath> {
    fs: FileManager<P>,
    storages: HashMap<TypeId, AssetStorage>,
}

impl<P: LogicalPath> ResourceManager<P> {
    /// Creates a new resource manager backed by the given file manager.
    pub fn new(fs: FileManager<P>) -> Self {
        Self {
            fs,
            storages: HashMap::new(),
        }
    }

    /// Loads an asset from disk via the [`Asset`] trait and returns a handle to it.
    pub fn load<A: Asset>(
        &mut self,
        path: P,
        file: &str,
    ) -> Result<Handle<A>, A::Error> {
        let full_path = self.fs.resolve(path, file)
            .map_err(|_| panic!("File resolution failed"))?;

        let asset = A::load(full_path)?;

        let type_id = TypeId::of::<A>();
        let storage = self.storages
            .entry(type_id)
            .or_insert_with(|| AssetStorage {
                next_id: 0,
                assets: HashMap::new(),
            });

        let id = storage.next_id;
        storage.next_id += 1;

        storage.assets.insert(id, Box::new(asset));

        Ok(Handle::new(id))
    }

    /// Stores a value directly (no file loading) and returns a handle to it.
    pub fn insert<T: 'static>(&mut self, value: T) -> Handle<T> {
        let type_id = TypeId::of::<T>();
        let storage = self.storages
            .entry(type_id)
            .or_insert_with(|| AssetStorage {
                next_id: 0,
                assets: HashMap::new(),
            });

        let id = storage.next_id;
        storage.next_id += 1;
        storage.assets.insert(id, Box::new(value));

        Handle::new(id)
    }

    /// Retrieves a reference to the resource behind `handle`, or `None` if missing.
    pub fn get<T: 'static>(&self, handle: Handle<T>) -> Option<&T> {
        let storage = self.storages.get(&TypeId::of::<T>())?;
        storage.assets
            .get(&handle.id)?
            .downcast_ref::<T>()
    }

    /// Removes and returns the resource behind `handle`, or `None` if missing.
    /// The returned value will be dropped by the caller, triggering GPU cleanup for types like `GpuMesh` or `Shader`.
    pub fn remove<T: 'static>(&mut self, handle: Handle<T>) -> Option<T> {
        let storage = self.storages.get_mut(&TypeId::of::<T>())?;
        storage.assets.remove(&handle.id)?
            .downcast::<T>().ok().map(|b| *b)
    }
}

/// Read-only access to resources by handle; implemented by [`ResourceManager`].
pub trait ResourceAccess {
    /// Retrieves a reference to the resource behind `handle`, or `None` if missing.
    fn get<T: 'static>(&self, handle: Handle<T>) -> Option<&T>;
}

impl<P: LogicalPath> ResourceAccess for ResourceManager<P> {
    fn get<T: 'static>(&self, handle: Handle<T>) -> Option<&T> {
        self.get(handle)
    }
}

/// Mutable access to resources; extends [`ResourceAccess`] with insert and remove.
pub trait ResourceStore: ResourceAccess {
    /// Stores a value and returns a handle to it.
    fn insert<T: 'static>(&mut self, value: T) -> Handle<T>;
    /// Removes and returns the resource behind `handle`, or `None` if missing.
    fn remove<T: 'static>(&mut self, handle: Handle<T>) -> Option<T>;
}

impl<P: LogicalPath> ResourceStore for ResourceManager<P> {
    fn insert<T: 'static>(&mut self, value: T) -> Handle<T> {
        self.insert(value)
    }
    fn remove<T: 'static>(&mut self, handle: Handle<T>) -> Option<T> {
        self.remove(handle)
    }
}

