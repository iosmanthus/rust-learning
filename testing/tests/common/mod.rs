extern crate testing;
#[test]
fn test_adder() {
    assert_eq!(4,testing::add_two(2));
}

pub fn setup() {
    println!("hello world");
}
