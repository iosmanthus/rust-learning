#[derive(Debug)]
struct Foo {
    a: i32,
    b: i32,
    c: i32,
}

impl Default for Foo {
    fn default() -> Self {
        Foo { a: 1, b: 2, c: 3 }
    }
}

fn main() {
    let foo: Option<Foo> = None;
    println!("{:?}", foo.unwrap_or_default());
}
