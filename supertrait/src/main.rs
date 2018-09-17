use std::fmt;
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let times = output.len();

        println!("{}", "*".repeat(times + 4));
        println!("*{}*", " ".repeat(times + 2));
        println!("* {} *", self);
        println!("*{}*", " ".repeat(times + 2));
        println!("{}", "*".repeat(times + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

fn main() {
    for (i, j) in (0..10).zip(1..11) {
        Point::new(i, j).outline_print();
        println!("");
    }
}
