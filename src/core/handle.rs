use std::marker::PhantomData;

/// Lightweight typed identifier for a resource stored in a [`ResourceManager`].
#[derive(Debug)]
pub struct Handle<T> {
    pub(crate) id: u32,
    _marker: PhantomData<T>,
}

impl<T> Handle<T> {
    pub(crate) fn new(id: u32) -> Self {
        Self { id, _marker: PhantomData }
    }
}

// Manual impls to avoid the T: Copy / T: Clone / T: PartialEq / T: Hash bounds
// that #[derive] would add. PhantomData<T> is always Copy regardless of T.

impl<T> Copy for Handle<T> {}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self { *self }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool { self.id == other.id }
}

impl<T> Eq for Handle<T> {}

impl<T> std::hash::Hash for Handle<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.id.hash(state); }
}
