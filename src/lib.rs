#![no_std]

use core::mem;
use core::pin::Pin;
use core::ptr;
use core::slice;

pub fn make_static<T: ?Sized>(shared_ref: &T) -> &'static T {
    unsafe { &*(shared_ref as *const T) }
}

pub fn make_static_mut<T: ?Sized>(shared_ref: &mut T) -> &'static mut T {
    unsafe { &mut *(shared_ref as *mut T) }
}

pub fn make_mut<T: ?Sized>(shared_ref: &T) -> &mut T {
    unsafe { &mut *(shared_ref as *const T as *mut T) }
}

pub fn copy<T>(shared_ref: &T) -> T {
    unsafe { ptr::read(shared_ref) }
}

pub fn as_bytes<T: ?Sized>(shared_ref: &T) -> &[u8] {
    unsafe {
        slice::from_raw_parts(
            shared_ref as *const T as *const u8,
            mem::size_of_val(shared_ref),
        )
    }
}

pub fn as_bytes_mut<T: ?Sized>(shared_ref: &mut T) -> &mut [u8] {
    unsafe {
        slice::from_raw_parts_mut(
            shared_ref as *mut T as *mut u8,
            mem::size_of_val(shared_ref),
        )
    }
}

pub fn destroy<T: ?Sized>(mut_ref: &mut T) {
    unsafe {
        ptr::drop_in_place(mut_ref);
    }
}

pub struct Sender<T> {
    pub inner: T,
}

impl<T> Sender<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

unsafe impl<T> Send for Sender<T> {}

pub fn unpin<T: ?Sized>(pinned_ref: Pin<&mut T>) -> &mut T {
    unsafe { Pin::into_inner_unchecked(pinned_ref) }
}
