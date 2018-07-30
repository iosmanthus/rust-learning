pub fn no() -> u32 {
    println!("hello world");
    10
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works()
    {
        assert_eq!(10,no());
    }
}
mod sub;
