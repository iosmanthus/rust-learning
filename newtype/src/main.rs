use std::fmt;
use std::fmt::Debug;
use std::ops::Deref;

struct Wrapper<T>(Vec<T>)
where
    T: Debug;

impl<T> fmt::Display for Wrapper<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T> Deref for Wrapper<T>
where
    T: Debug,
{
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

fn main() {
    let wrapper = Wrapper(vec![
        String::from("I"),
        String::from("am"),
        String::from("a"),
        String::from("student"),
    ]);
    println!("{}", wrapper);
}
