pub mod adder;
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be no less than 1, got {}.", value);
        }
        else if value>100 {
            panic!("Guess value must be no larger than 100, got {}.",value);
        }

        Guess {
            value
        }
    }
}

pub fn print_and_return_10(value: i32) ->i32{
    println!("I got an value {}", value);
    10
}

pub fn add_two(vaule: i32) -> i32{
    vaule + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(2,add_two(0));
    }
    #[test]
    fn test2() {
        assert_eq!(3,add_two(1));
    }
    #[test]
    #[ignore]
    fn test3() {
        assert_eq!(5,add_two(2));
    }
}
