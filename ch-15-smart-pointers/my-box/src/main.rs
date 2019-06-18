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

fn main() {
    let x = 5;
    let boxed_x = MyBox::new(x);
    assert_eq!(5, x);

    // this wouldn't work if we didn't implement the Deref trait
    // and is equivalent to *(y.deref())
    assert_eq!(5, *boxed_x);
}
