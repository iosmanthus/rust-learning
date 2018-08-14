pub fn add_two(x: u32) -> u32 {
    x + 2
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
