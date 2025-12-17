// Test that two library crates with the same name can export macros with the same name
// without causing interference when both are used in another crate via Cargo's rename feature.
// This verifies the fix for an issue present in rustc 1.42.0 and fixed by rustc 1.44.0-nightly.

use run_make_support::cargo;

fn main() {
    // Build and run the consumer crate which uses both renamed libraries
    cargo()
        .current_dir("consumer")
        .arg("run")
        .run();
}
