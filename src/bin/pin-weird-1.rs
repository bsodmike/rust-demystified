#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Error;
use std::fmt::Debug;
use std::pin::Pin;
use std::ptr;

extern crate tutorials;

// NOTE: Warning: avoid the pin! macro unless you have a very good understanding of what it does. This macro is sometimes over-eagerly suggested by the compiler as the “solution” to pinning-related errors (as you may already have noticed in the compiler error listed in the previous chapter). However, this macro is only applicable under specific conditions and often not the right tool for the job.
// https://sander.saares.eu/2024/11/06/why-is-stdpinpin-so-weird/

pub struct BagOfApples {
    count: usize,

    // In the example code, this self-reference is mostly useless.
    // This is just to keep the example code simple – the emphasis is
    // on the effects of pinning, not why a type may be designed to need it.
    self_reference: *mut BagOfApples,

    _require_pin: std::marker::PhantomPinned,
}

impl BagOfApples {
    pub fn new() -> Self {
        BagOfApples {
            count: 0,
            // We cannot set this here because we have not yet
            // created the BagOfApples – there is nothing to reference.
            self_reference: ptr::null_mut(),
            _require_pin: std::marker::PhantomPinned,
        }
    }

    /// Call this after creating a BagOfApples to initialize the instance.
    pub fn initialize(mut self: Pin<&mut Self>) {
        // SAFETY: BagOfApples requires pinning and we do not allow
        // the obtained pointer to be exposed outside this type, so
        // we know it always points to a valid value and therefore it
        // is safe to store the pointer. We have also reviewed the code of
        // this function to ensure that we do not move the BagOfApples
        // instance via the reference we obtain from here nor via the pointer.
        let self_mut = unsafe { self.as_mut().get_unchecked_mut() };

        self_mut.self_reference = self_mut;
    }

    pub fn count(&self) -> usize {
        assert!(
            !self.self_reference.is_null(),
            "BagOfApples is not initialized"
        );

        // SAFETY: Simple read-only access to the count field, which
        // is safe. We do it via the pointer for example purposes.
        unsafe { (*self.self_reference).count }
    }
}

pub fn main() -> Result<(), Error> {
    let mut bag = Box::pin(BagOfApples::new());
    bag.as_mut().initialize();
    println!("Apple count: {}", bag.count());

    Ok(())
}
