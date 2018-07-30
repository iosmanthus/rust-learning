pub mod shape {
    pub struct Rectangle {
        length: i32,
        width: i32,
    }
    impl Rectangle {
        pub fn new(length: i32, width: i32) -> Rectangle {
            Rectangle { length, width }
        }
        pub fn can_hold(&self, another: &Rectangle) -> bool {
            self.length > another.length && self.width > another.width
        }
    }
} /* shape */

#[cfg(test)]
mod tests {
    fn greeting() -> String {
        format!("Hello!")
    }
    #[test]
    fn it_works() {
        let result = greeting();
        assert!(result.contains("iosmanthus"), "Fuck! Where is my name!");
    }
}
