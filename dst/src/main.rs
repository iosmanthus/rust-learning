use std::fmt::Display;

fn generic<T: ?Sized>(t: &T) -> &T {
    t
}

trait Foo {
    fn foo(&self);
}

impl<T> Foo for T
where
    T: Display,
{
    fn foo(&self) {
        println!("{}", self);
    }
}

fn main() {
    let x = &String::from("hello world") as &Foo;
    generic(x);
}
