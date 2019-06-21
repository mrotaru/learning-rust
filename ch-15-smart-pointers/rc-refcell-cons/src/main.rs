use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let value = Rc::new(RefCell::new(10));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // Why clone &a ? Because if we used  just `a`, then it would
    // be moved into `b`. And if we used just a reference, without
    // cloning, then it would not be reference counted - we'd probably
    // have to use lifetimes
    let b = Cons(Rc::new(RefCell::new(15)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(20)), Rc::clone(&a));
    /*
    Result:

    b (15) -
             \
              > --> a (10) --> Nil
             /
    c (20) -

    */
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);

    // Now, if we change the value that is referenced by `a`, it should be
    // reflected in `b` and `c`. The Rc will be automatically dereferenced
    // to the inner RefCell, on which `borrow_mut()` is called:
    *value.borrow_mut() = 42;
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
}
