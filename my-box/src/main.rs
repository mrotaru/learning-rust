use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl <T> Deref for MyBox<T> {
    type Target = T; // declaring an associated type (?)

    fn deref(&self) -> &T {
        // We return a reference, because if we returned the value
        // itself, it would be moved out; but we want smart pointers
        // to own their values.
        &self.0 // this .0 syntax, don't think I've seen it before (?)
    }

}

// If we get a ref to something that is a ref to the required type (&str), and
// it implements the Deref trait, Rust will apply deref automatically. A String
// can be derefed to a string slice:
// https://doc.rust-lang.org/std/string/struct.String.html#impl-Deref.

// So, in a way, this auto-deref functionality is multi-level; if Rust can
// obtain what it wants (in this case, a string slice) by applying deref more
// than once, it will do it.
fn give_me_a_string_slice(slice: &str) {
    println!("Delicious slice: {}", slice);
}

fn main() {
    let x = 5;
    let boxed_x = MyBox::new(x);
    assert_eq!(5, x);

    // this wouldn't work if we didn't implement the Deref trait
    // and is equivalent to *(y.deref())
    assert_eq!(5, *boxed_x);

    let slice = MyBox::new(String::from("cake"));
    give_me_a_string_slice(&slice);
    // without auto-deref, we'd have to build the slice from a String manually:
    give_me_a_string_slice(&(*slice)[..]); // eew, ugly

    // a box, inside a box, which can be derefed:
    let box_in_a_box = MyBox::new(slice);
    give_me_a_string_slice(&box_in_a_box); // wow, amaze
}
