#[derive(Debug, PartialEq)]
struct Foo(String);

#[derive(Debug, PartialEq)]
enum Bar {
    Variant1(i32),
    Variant2(i32),
}

fn main() {
    let bar1_1 = Bar::Variant1(1);
    let bar1_2 = Bar::Variant1(1);
    assert_eq!(bar1_1, bar1_2);
    let bar2 = Bar::Variant2(1);
    assert_eq!(bar1_1, bar2);
}
