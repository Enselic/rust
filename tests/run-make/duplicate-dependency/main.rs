use foo::Foo;
use re_export_foo::into_foo;

struct Bar;

impl From<Bar> for Foo {
    fn from(_: Bar) -> Self {
        Foo
    }
}

fn main() {
    into_foo(Bar);
}
