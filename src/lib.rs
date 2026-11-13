#![no_std]

extern crate alloc;

use core::ptr::NonNull;

pub fn with<A>(alloc: &A, scope: impl for<'alloc> FnOnce(&Allocator<'alloc, A>)) {
    let ptr: NonNull<Allocator<A>> = NonNull::from_ref(alloc).cast();
    // SAFETY: `Allocator<A>` is `repr(transparent)` and therefore has the same layout as `A`.
    scope(unsafe { ptr.as_ref() });
}

/// A branded wrapper around an allocator.
/// Created using [`with`].
#[repr(transparent)]
pub struct Allocator<'alloc, A: ?Sized> {
    brand: PhantomInvariantLifetime<'alloc>,
    inner: A,
}

/// A box that doesn't store its allocator. Note that dropping it leaks memory;
/// to destroy it correctly, use [`Box::into_inner`].
pub struct Box<'alloc, T: ?Sized> {
    brand: PhantomInvariantLifetime<'alloc>,
    ptr: NonNull<T>,
}

impl<'alloc, T> Box<'alloc, T> {
    /// Allocates memory in the given allocator then places `x` into it.
    pub fn new_in<A>(x: T, alloc: &Allocator<'alloc, A>) -> Self
    where
        A: std::Allocator + ?Sized,
    {
        let brand = core::marker::PhantomData;
        let ptr = std::Box::into_non_null_with_allocator(std::Box::new_in(x, &alloc.inner)).0;
        Self { brand, ptr }
    }

    /// Consumes the `Box`, returning the wrapped value.
    #[expect(clippy::needless_pass_by_value, reason = "consumes the wrapped value")]
    pub fn into_inner<A>(boxed: Self, alloc: &Allocator<'alloc, A>) -> T
    where
        A: std::Allocator + ?Sized,
    {
        // SAFETY: `boxed.ptr` was acquired using `Box::into_non_null_with_allocator`
        // and the `Allocator` lifetime shenanigans ensure that the same allocator is used.
        *unsafe { std::Box::from_non_null_in(boxed.ptr, &alloc.inner) }
    }
}

impl<T: ?Sized> core::ops::Deref for Box<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: `boxed.ptr` was acquired using `Box::into_non_null_with_allocator`,
        // which returns a valid pointer.
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized> core::ops::DerefMut for Box<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `boxed.ptr` was acquired using `Box::into_non_null_with_allocator`,
        // which returns a valid pointer.
        unsafe { self.ptr.as_mut() }
    }
}

type PhantomInvariantLifetime<'alloc> = core::marker::PhantomData<fn(&'alloc ()) -> &'alloc ()>;

mod std {
    pub use alloc::alloc::Allocator;
    pub use alloc::boxed::Box;
}
