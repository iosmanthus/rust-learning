fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    let b = vec![0, 1001, 01010, 10];
    println!("{}", largest(&b));
}
