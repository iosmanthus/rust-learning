use std::cell::RefCell;
use std::rc::{Rc, Weak};
/* #[derive(Debug, Clone)] */
/*enum List {
  Cons(i32, RefCell<Rc<List>>),
  Nil,
  }

  impl List {
  fn tail(&self) -> Option<&RefCell<Rc<List>>> {
  match *self {
  Cons(_, ref item) => Some(item),
  Nil => None,
  }
  }
  }

  use List::{Cons, Nil};
  fn main() {
  let a = Rc::new(Cons(0, RefCell::new(Rc::new(Nil))));
  println!("a initial rc count = {}", Rc::strong_count(&a));
  println!("a next item = {:?}", a.tail());

  let b = Rc::new(Cons(1, RefCell::new(Rc::clone(&a))));
  println!("a rc count after b creation = {}", Rc::strong_count(&a));
  println!("b initial rc count = {}", Rc::strong_count(&b));
  println!("b next item = {:?}", b.tail());

  if let Some(item) = a.tail() {
 *item.borrow_mut() = Rc::clone(&b);
 }
 println!("b rc count after changing b = {}", Rc::strong_count(&b));
 println!("a rc count after changing a = {}", Rc::strong_count(&a));
 drop(b);
 println!("a rc count after changing a = {}", Rc::strong_count(&a));
 [>let mut a = Rc::new(String::from("dlm"));
 let b = Rc::clone(&a);
 let c = Rc::clone(&b);
 println!(
 "a = {}\nb = {}\nc = {}",
 Rc::strong_count(&a),
 Rc::strong_count(&b),
 Rc::strong_count(&c)
 );<]
 }*/
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("After creating `leaf`");
    println!(
        "leaf's \
         strong count = {}
         weak count = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    println!("");

    {
        let branch = Rc::new(Node {
            value: 2,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        println!("After creating `branch`");
        println!(
            "branch's \
         strong count = {}
         weak count = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        println!("");

        /////////////
        //  NOTE!  //
        /////////////
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("After linking `branch` to `node`");
        println!(
            "leaf's \
         strong count = {}
         weak count = {}
         parent weak count = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
        println!(
            "branch's \
         strong count = {}
         weak count = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        println!("");
    }
    println!("After dropping `branch`");
    println!(
        "leaf's \
         strong count = {}
         weak count = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    println!("");
}
