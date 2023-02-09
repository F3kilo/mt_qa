fn main() {}

fn _subtype_coersion() {
    // Type
    let _boxed: Box<dyn ToString> = Box::new(42);

    // Lifetimes
    let s = String::new();
    let mut _s_ref: &str = &s; // _s_ref: &'a str
    _s_ref = "abc"; // _s_ref: &'a str = &'static str
}

fn _mut_coersion() {
    let a = &mut 42;
    let _b: &i32 = a;
}

fn _deref_coersion() {
    // if T: impl Deref<Target = U>, then (&T -> &U)
    let mut s = String::new();
    let _s_ref: &str = &s;
    let _s_ref: &mut str = &mut s;
}

fn _transitive_coersion() {
    // (T1 -> T3), if (T1 -> T2) and (T2 -> T3)
    let mut s = String::new();
    let _s_ref: &str = &mut s;
}

fn _function_coersion() {
    fn foo<T>() -> usize {
        std::mem::size_of::<T>()
    }

    let mut _x: fn() -> usize = foo::<i32>;
    _x = foo::<u32>;

    // and non-capturing closures
    _x = || { 42 };
}
