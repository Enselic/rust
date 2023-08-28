// run-pass
// check-run-results

fn main() {
    println!(include!("silly-file-names/<leading-lt"));
    println!(include!("silly-file-names/trailing-gt>"));
}
