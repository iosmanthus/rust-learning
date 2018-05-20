fn gen_fib(index: usize) -> Vec<usize> {
    let mut fibs: Vec<usize> = vec![0, 1];
    if index > 1 {
        for i in 0..index - 1 {
            let next = fibs[i] + fibs[i + 1];
            fibs.push(next);
        }
    }
    fibs
}
fn main() {
    let fibs = gen_fib(0);
    for n in &fibs {
        print!("{} ", n);
    }
    println!("");
}
