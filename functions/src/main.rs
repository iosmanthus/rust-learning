//type Multiplier = Fn(i32) -> i32;

/*fn multiplier(c: i32) -> Box<Multiplier> {
    Box::new(move |x| c * x)
}*/
fn multiplier(c: i32) -> impl FnOnce(i32) -> i32 {
    move |x| c * x
}

fn main() {
    let m = multiplier(3);
    println!("{}", m(4));
}
