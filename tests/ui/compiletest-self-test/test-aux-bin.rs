//@ aux-bin: print-it-works.rs
//@ run-pass

fn main() {
    let output = std::process::Command::new("auxiliary/bin/print-it-works").output().unwrap();

    eprintln!("{:?}", std::str::from_utf8(&output.stderr));

    assert_eq!(output.stdout, b"it works\n");
}
