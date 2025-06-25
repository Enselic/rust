// issue 65419 - Attempting to run an `async fn` after completion mentions coroutines when it should
// be talking about `async fn`s instead. Regression test added to make sure coroutines still
// panic when resumed after completion.

//@ run-fail
//@ edition:2018

fn main() {
}
