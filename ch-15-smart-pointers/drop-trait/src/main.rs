struct Foo {
    data: String,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo with data: `{}`!", self.data)
    }
}

fn main() {
    println!("before");
    let _foo_1 = Foo { data: String::from("foo 1")};
    let _foo_2 = Foo { data: String::from("foo 2")};
    let _foo_3 = Foo { data: String::from("foo 3")};

    // Code below won't compile. Values are dropped automatically when they go
    // out of scope. So, if we could explicitly drop it, the drop function
    // (destructor) will be called twice - when we call it, and when it goes out
    // of scope. A "double-free", in other words.
    // _foo_3.drop();

    // Can use std::mem::drop to prematurely drop a value. This is in the
    // prelude, so we can just call it without namespacing. This method is
    // actually very simple: https://doc.rust-lang.org/std/mem/fn.drop.html
    // Because we moved the value into the function, as soon as `drop` is done,
    // the destructor (implemented via the `Drop` trait) will be called simply
    // because the value goes out of scope, and there are no other references to
    // it - because it was _moved_ into the function. Beautiful !
    drop(_foo_3);

    println!("after");

    // Note that the drop function will be called after the "after" string is
    // printed, and in reverse order. So, foo_2 will be dropped before foo_1.
    // But foo_3 is dropped explicitly, so it's destructor will be called before
    // the other ones.
}
