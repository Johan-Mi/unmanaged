use crate::{Allocator, NonNull, PhantomInvariantLifetime, std};
use core::mem::ManuallyDrop;

/// A vector that doesn't store its allocator. Note that dropping it leaks memory;
/// to destroy it correctly, use [`Vec::into_managed`].
pub struct Vec<'alloc, T> {
    brand: PhantomInvariantLifetime<'alloc>,
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

impl<'alloc, T> Vec<'alloc, T> {
    #[must_use]
    /// Constructs a new, empty `Vec<T>`.
    ///
    /// The vector will not allocate until elements are pushed onto it.
    pub const fn new() -> Self {
        Self {
            brand: core::marker::PhantomData,
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }

    pub fn into_managed<A>(self, alloc: Allocator<'alloc, A>) -> std::Vec<T, A>
    where
        A: std::Allocator,
    {
        // SAFETY: `self` is consumed, so no double free or use after free can occur.
        unsafe { Self::dup_to_managed(&self, alloc) }
    }

    /// # Safety
    ///
    /// This creates another vector that owns the same allocation.
    /// Do not use both `self` and the newly returned vector.
    unsafe fn dup_to_managed<A>(&self, alloc: Allocator<'alloc, A>) -> std::Vec<T, A>
    where
        A: std::Allocator,
    {
        // SAFETY: `self.{ptr, len, cap}` always form a valid vector.
        unsafe { std::Vec::from_parts_in(self.ptr, self.len, self.cap, alloc.inner) }
    }

    /// Appends an element to the back of a collection.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` _bytes_.
    pub fn push<A>(&mut self, value: T, alloc: Allocator<'alloc, A>)
    where
        A: std::Allocator,
    {
        // SAFETY: `managed` is never dropped; either everything succeeds and
        // `into_parts_with_allocator` consumes it without deallocating it,
        // or `push` panics and `managed` is leaked,
        // leaving `self` as the sole owner of the allocation.
        let mut managed = ManuallyDrop::new(unsafe { self.dup_to_managed(alloc) });
        managed.push(value);
        let (ptr, len, cap, _) = ManuallyDrop::into_inner(managed).into_parts_with_allocator();
        *self = Self {
            brand: core::marker::PhantomData,
            ptr,
            len,
            cap,
        };
    }
}

impl<T> Default for Vec<'_, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> core::ops::Deref for Vec<'_, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        // SAFETY: `self.{ptr, len, cap}` always form a valid vector.
        unsafe { NonNull::slice_from_raw_parts(self.ptr, self.len).as_ref() }
    }
}

impl<T> core::ops::DerefMut for Vec<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `self.{ptr, len, cap}` always form a valid vector.
        unsafe { NonNull::slice_from_raw_parts(self.ptr, self.len).as_mut() }
    }
}
