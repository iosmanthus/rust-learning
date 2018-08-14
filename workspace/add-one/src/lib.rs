pub fn add_one(x: u32) -> u32 {
    x + 1
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
