#![no_std]

extern crate alloc;

mod boxed;
pub use boxed::Box;
mod vec;
pub use vec::Vec;

use core::ptr::NonNull;

pub fn with<A>(alloc: A, scope: impl for<'alloc> FnOnce(Allocator<'alloc, A>)) {
    let brand = core::marker::PhantomData;
    let inner = alloc;
    scope(Allocator { brand, inner });
}

/// A branded wrapper around an allocator.
/// Created using [`with`].
pub struct Allocator<'alloc, A> {
    brand: PhantomInvariantLifetime<'alloc>,
    inner: A,
}

// TODO: change this bound from `&A` to `A: AllocatorClone`
// when `allocator_ext` is stabilized
#[expect(
    clippy::expl_impl_clone_on_copy,
    reason = "false positive; `Clone` and `Copy` are only implemented for `Allocator<&A>`. Note the reference"
)]
impl<A> Clone for Allocator<'_, &A> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<A> Copy for Allocator<'_, &A> {}

type PhantomInvariantLifetime<'alloc> = core::marker::PhantomData<fn(&'alloc ()) -> &'alloc ()>;

mod std {
    pub use alloc::alloc::Allocator;
    pub use alloc::boxed::Box;
    pub use alloc::vec::Vec;
}
