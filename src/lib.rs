#![no_std]

extern crate alloc;

mod boxed;
pub use boxed::Box;

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

type PhantomInvariantLifetime<'alloc> = core::marker::PhantomData<fn(&'alloc ()) -> &'alloc ()>;

mod std {
    pub use alloc::alloc::Allocator;
    pub use alloc::boxed::Box;
}
