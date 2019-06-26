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

    // Deref coercion is applied to function and method params, when types mismatch
    // and the argument is a reference that can deref into the required type.
    // println!("{}", a + boxed); // won't work; deref coercions apply only to refs; `boxed` is not a ref
    println!("add operator: {}", a + *boxed); // will work
    // println!("{}", add(&a, boxed)); // won't work - `boxed` is not a reference, so will not be derefed
    println!("add fn: {}", add(&a, &boxed)); // will work

    // Deref coercion can be applied recursively
    let boxed_in_a_box = Box::new(&boxed);
    println!("add fn, boxed in a box: {}", add(&a, &boxed_in_a_box));

    impl<T> Deref for MyPointer<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.value
        }
    }
    
    let b = Rc::new(MyPointer {
        value: 42,
    });

    println!("add fn with MyPointer: {}", add(&a, &b)); // will work, because b can deref to &int (self.value is int, and Deref returns &self.value)
}
