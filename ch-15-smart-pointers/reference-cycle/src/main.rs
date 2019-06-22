use std::rc::Rc;
use std::cell::RefCell;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            List::Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a rc: {}", Rc::strong_count(&a));
    println!("a tail: {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc after creating b: {}", Rc::strong_count(&a));
    println!("b rc: {}", Rc::strong_count(&b));
    println!("b tail: {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("a rc: {}", Rc::strong_count(&a));
    println!("b rc: {}", Rc::strong_count(&b));

    // Calling tail will result in stack overflow:
    // println!("a tail: {:?}", a.tail());
}
