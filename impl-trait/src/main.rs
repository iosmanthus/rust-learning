type Multiplier = Fn(i32) -> i32;

fn multiplier(c: i32) -> Box<Multiplier> {
    if c > 3 {
        Box::new(move |x| (c - 3) * x)
    } else {
        Box::new(move |x| c * x)
    }
}

fn main() {
    let m = multiplier(3);
    println!("{}", m(3));
}
