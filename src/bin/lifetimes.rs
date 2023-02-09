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

fn _elided(_s: &str) {}

fn _elided2(_s: &str) -> &str {
    todo!()
}

fn _elided3(_s1: &str) -> RefWrapper<i32> {
    todo!()
}

// fn not_elided2(_s1: &str, _s2: &str) -> &str {
//     todo!()
// }

struct _A;

impl _A {
    fn _elided(&self, _s2: &str) -> &str {
        todo!()
    }

    fn _elided2(&self, _s2: &str) -> RefWrapper<i32> {
        todo!()
    }
}

// struct ClosureWrapper<F>
// where
//     F: Fn(&str, &str) -> &str,
// {
//     closure: F,
// }
