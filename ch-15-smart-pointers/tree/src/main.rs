use std::rc::{Rc, Weak};
use std::cell::RefCell;

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
    print_counts("leaf", &leaf);

    let branch = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    println!("--- after creating branch ---");
    print_counts("leaf", &leaf);
    print_counts("branch", &branch);

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("--- after setting leaf parent to branch ---");
    print_counts("leaf", &leaf);
    print_counts("branch", &branch);
    // println!("leaf: {:?}", leaf);
    // println!("brach: {:?}", branch);
    // println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());

    fn print_counts (name: &str, rc: &Rc<Node>) {
        println!(
            "{: >6} strong = {}, weak = {}", // https://doc.rust-lang.org/std/fmt/#syntax
            name,
            Rc::strong_count(&rc),
            Rc::weak_count(&rc),
        );
    } 
}
