use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

trait SquareSum<T = Self> {
    fn square_sum(lhs: T, rhs: T) -> T;
}

trait Distance<T> {
    fn distance(&self, remote: &T) -> i32;
}

impl SquareSum for i32 {
    fn square_sum(lhs: i32, rhs: i32) -> i32 {
        lhs.pow(2) + rhs.pow(2)
    }
}

impl Distance<(i32, i32)> for Point {
    fn distance(&self, remote: &(i32, i32)) -> i32 {
        <i32 as SquareSum>::square_sum(self.x - remote.0, self.y - remote.1)
    }
}

impl Distance<Point> for Point {
    fn distance(&self, remote: &Point) -> i32 {
        <i32 as SquareSum>::square_sum(self.x - remote.x, self.y - remote.y)
    }
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

    println!("c = {:?}", c);

    let d = c + (2, 3);
    println!("d = {:?}", d);

    println!("|d -> (1, 2)| = {}", d.distance(&(1, 2)));
    println!("|d -> d| = {}", d.distance(&d));
}
