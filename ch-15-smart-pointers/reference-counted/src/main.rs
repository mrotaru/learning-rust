use std::rc::Rc;

fn main() {
    enum BoxList {
        Cons(char, Box<BoxList>),
        Nil,
    }
    use BoxList::{Cons, Nil};
    let a = Cons('a', Box::new(Nil));
    // trying to create a shared object; so 'a' is shared by b and c.
    // This isn't going to work, because a is moved into b, and moved
    // values cannot be used after being moved:
    // let b = Cons('b', Box::new(a));
    // let c = Cons('c', Box::new(a));

    // We must use the `Rc` (reference counted) smart pointer.
    enum RcList {
        Cons(char, Rc<RcList>),
        Nil,
    }
    use RcList::{Cons as RcCons, Nil as RcNil};
    let a = Rc::new(RcCons('a', Rc::new(RcNil)));
    // note: `clone` DOES NOT make a deep clone, as you might expect
    // From docs of Rc implementation of the 'Clone' trait: 
    // "creates another pointer to the same inner value, increasing the
    // strong reference count."
    // https://doc.rust-lang.org/std/rc/struct.Rc.html#impl-Clone
    // https://doc.rust-lang.org/std/clone/trait.Clone.html
    let b = Rc::new(RcCons('b', Rc::clone(&a)));
    {
        let c = Rc::new(RcCons('c', Rc::clone(&a)));
        println!("Reference count 1: {}", Rc::strong_count(&a)); // => 3
    }
    println!("Reference count 2: {}", Rc::strong_count(&a)); // => 2, because c went out of scope, so reference count was decremented
}
