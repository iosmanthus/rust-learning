use std::rc::Rc;
#[derive(Debug, Clone)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
/*impl Iterator for List {
  type Item = i32;
  fn next(&mut self) -> Option<Self::Item> {
  let (value, rest) = match self {
  Cons(value, rest) => (value.clone(), rest.clone()),
  Nil => return None,
  };
 *self = *rest;
 Some(value)
 }
 }*/

use List::{Cons, Nil};
fn main() {
    /*let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
      let vec = list.collect::<Vec<_>>();
      vec.iter().for_each(|i| {
      print!("{} ", i);
      });*/
    //let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    //println!("count after creating a = {}", Rc::strong_count(&a));
    ;
}
