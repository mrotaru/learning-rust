use std::rc::Rc;
use std::ops::Deref;

struct MyPointer<T> {
    value: T,
}

impl<T> MyPointer<T> {
    fn new(x: T) -> MyPointer<T> {
        MyPointer {
            value: x,
        }
    }
}

fn add(a: &i32, b: &i32) -> i32 {
    a + b
}

fn main() {
    // Box - "take value, package it in a box, send it to some storage facility;
    // keep receipt". The receipt is the pointer; storage facility is heap
    // memory. Works for simple values (like ints), not just structures -
    // although it is rarely useful to box scalars.
    let a = 42;
    let boxed = Box::new(a);

    // When Rust knows what type is needed, and a value is not of that type, it
    // will check whether it can "deref" to the required type. Not sure why code
    // below doesn't work; Box implements deref, but it's not applied.
    // println!("{}", a + boxed); // won't work; why isn't deref coercion applied ?
    println!("{}", a + *boxed); // will work
    println!("{}", add(&a, &boxed)); // will work

    impl<T> Deref for MyPointer<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.value
        }
    }
    
    let b = Rc::new(MyPointer {
        value: 42,
    });

    println!("{}", add(&a, &b)); // will work, because b can deref to &int (self.value is int, and Deref returns &self.value)
}
