fn main() {
    let a = &42;

    foo(a);
    bar(a);

    let b = RefWrapper(a);

    foo(b);
    // bar(b); // ERROR!
}

fn foo<'a, T: 'a>(_t: T) {}
fn bar<T>(_t: &T) {}

struct RefWrapper<'a, T>(&'a T);