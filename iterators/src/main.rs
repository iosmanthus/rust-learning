fn main() {
    let s = "how many roads must a man walk down\n"
        .trim()
        .split_whitespace();
    for w in s {
        println!("{}", w);
    }
}
