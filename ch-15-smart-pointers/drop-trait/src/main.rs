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
    let foo_1 = Foo { data: String::from("foo 1")};
    let foo_2 = Foo { data: String::from("foo 2")};
    println!("after");
    // note that the drop function will be called after
    // the "after" string is printed, and in reverse order.
    // So, foo_2 will be dropped before foo_1
}
