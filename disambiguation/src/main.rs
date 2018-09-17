trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    #[allow(dead_code)]
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("{}", <Dog as Animal>::baby_name());
}
