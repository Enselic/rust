// Tests that if mulitple crate_level_only lints are active at the same time,
// both of them are reported as errors.

#[allow(uncommon_codepoints)]
//~^ ERROR allow(uncommon_codepoints) is ignored unless specified at crate level [unused_attributes]
fn main() {
    #[allow(non_ascii_idents)]
    //~^ ERROR allow(non_ascii_idents) is ignored unless specified at crate level [unused_attributes]
    const åååååå: f64 = 0.000001;
}
