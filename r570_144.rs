// SPDX-License-Identifier: GPL-2.0

//! Firmware bindings.
//!
//! Imports the generated bindings by `bindgen`.
//!
//! This module may not be directly used. Please abstract or re-export the needed symbols in the
//! parent module instead.

#![allow(
    dead_code,
    clippy::all,
    clippy::undocumented_unsafe_blocks,
    clippy::ptr_as_ptr,
    clippy::ref_as_ptr,
    missing_docs,
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    improper_ctypes,
    unreachable_pub,
    unsafe_op_in_unsafe_fn
)]
use kernel::ffi;
use pin_init::MaybeZeroable;
use zerocopy_derive::{
    FromBytes,
    IntoBytes,
    KnownLayout,
    Immutable, //
};

include!("r570_144_bindings.rs");

// SAFETY: This type has a size of zero, so its inclusion into another type should not affect their
// ability to implement `Zeroable`.
unsafe impl<T> kernel::prelude::Zeroable for __IncompleteArrayField<T> {}

/// Monomorphic version of [`__IncompleteArrayField<PACKED_REGISTRY_ENTRY>`].
///
/// zerocopy's `IntoBytes` derive can only run its compile-time no-padding check
/// on a concrete type; for the generic `__IncompleteArrayField<T>` it falls back
/// to requiring every field be `Unaligned`, which `PACKED_REGISTRY_ENTRY`
/// does not satisfy. Specializing to a concrete type lets the derive succeed.
#[repr(C)]
#[derive(Debug, Default, MaybeZeroable, FromBytes, Immutable, KnownLayout, IntoBytes)]
pub struct __IncompletePackedRegistryEntry(
    ::core::marker::PhantomData<PACKED_REGISTRY_ENTRY>,
    [PACKED_REGISTRY_ENTRY; 0],
);
impl __IncompletePackedRegistryEntry {
    #[inline]
    pub const fn new() -> Self {
        __IncompletePackedRegistryEntry(::core::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const PACKED_REGISTRY_ENTRY {
        self as *const _ as *const PACKED_REGISTRY_ENTRY
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut PACKED_REGISTRY_ENTRY {
        self as *mut _ as *mut PACKED_REGISTRY_ENTRY
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[PACKED_REGISTRY_ENTRY] {
        ::core::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [PACKED_REGISTRY_ENTRY] {
        ::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
