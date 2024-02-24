//@ compile-flags: -Z print-type-sizes --crate-type=lib
//@ build-pass
//@ ignore-pass

#![feature(coroutines, coroutine_trait)]
#![allow(large_assignments)]

use std::ops::Coroutine;

fn coroutine<const C: usize>(array: [u8; C]) -> impl Coroutine<Yield = (), Return = ()> {
    move |()| {
        yield ();
        let _ = array;
    }
}

pub fn foo() {
    let _ = coroutine([0; 8192]);
}
