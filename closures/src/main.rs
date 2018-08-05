use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    result: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            result: HashMap::new(),
        }
    }
    pub fn value(&mut self, arg: u32) -> u32 {
        if let Some(v) = self.result.get(&arg) {
            return *v;
        }
        let v = (self.calculation)(arg);
        self.result.insert(arg, v);
        v
    }
}
fn main() {
    let mut cacher = Cacher::new(move |x| {
        println!("Waiting");
        thread::sleep(Duration::from_secs(1));
        x
    });
    println!("{}", cacher.value(10));
    println!("{}", cacher.value(10));
}
