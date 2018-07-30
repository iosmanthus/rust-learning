extern crate testing;

mod common;

#[test]
fn printable() {
    common::setup();
    assert_eq!(23,testing::add_two(21));
}
