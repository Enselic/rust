//! Tests that if mulitple crate_level_only lints are active at the same time,
//! both of them are reported as errors, but only one time each.

#![deny(unused_attributes)]
#![allow(dead_code)]

#[allow(uncommon_codepoints)]
//~^ ERROR allow(uncommon_codepoints) is ignored unless specified at crate level [unused_attributes]
fn main() {
    #[allow(non_ascii_idents)]
    //~^ ERROR allow(non_ascii_idents) is ignored unless specified at crate level [unused_attributes]
    //~| ERROR allow(non_ascii_idents) is ignored unless specified at crate level [unused_attributes]
    const ÅÅÅÅ: f64 = 0.000001;
}
