use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Point;
    fn add(self, rhs: (i32, i32)) -> Point {
        Point::new(self.x + rhs.0, self.y + rhs.1)
    }
}

fn main() {
    let a = Point::new(1, 2);
    let b = Point::new(-1, -2);
    let c = a + b;
    println!("{:?}", c);
    println!("{:?}", c + (2, 3));
}
