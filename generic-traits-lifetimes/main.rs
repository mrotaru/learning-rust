use std::fmt::Display;

fn main () {
    sort();
    structs();
    different_types();
    traits();
    conditional_trait_methods();
    lifetimes();
}

fn sort () {
    let list = vec![2, 5, 20, 4];
    println!("Largest number is {}", largest(&list));
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &n in list.iter() {
            if n > largest {
                largest = n;
            }
        }
        largest
    }
    println!("Largest nr: {}", largest(&vec![1,5,2]));
    println!("Largest char: {}", largest(&vec!['a', 'c', 'x', 'b']));
}

fn structs () {
    struct Point<T> {
        x: T,
        y: T,
    }

    // defining a method
    impl<T> Point<T> {
        fn get_x(&self) -> &T {
            &self.x
        }
    }
    let p = Point { x: 3.0, y: 4.0 };
    println!("x: {}", p.get_x());

    // defining a method for a particular T
    impl Point<f32> {
        fn dist_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    println!("Distance: {}", p.dist_from_origin());
}

fn different_types () {
    struct Point<A, B> {
        x: A,
        y: B,
    }
    impl<T1, T2> Point<T1, T2> {
        fn foo<T3, T4>(self, other: Point<T3, T4>) -> Point<T1, T4> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "foo", y: 'c' };
    let p3 = p1.foo(p2);
    println!("p3.x: {}, p3.y: {}", p3.x, p3.y);
}

fn traits () {
    pub trait Foo {
        fn bar(&self) -> String {
            String::from("<default>")
        }
        fn boz(&self) -> String {
            format!("[ boz {} ]", self.bar())
        }
    }

    pub struct A {
        pub x: String,
    }
    impl Foo for A {
        // fn bar(&self) -> String {
        //     format!("bar A {}", self.x)
        // }
    }

    pub struct B {
        pub y: i32,
    }
    impl Foo for B {
        fn bar(&self) -> String {
            format!("bar B {}", self.y)
        }
    }

    let a1 = A { x: String::from("a1")};
    let a2 = A { x: String::from("a2")};
    let b1 = B { y: 42 };
    let b2 = B { y: 43 };
    println!("{}, {}, {}, {}, {}, {}", a1.bar(), a1.boz(), a2.bar(), b1.bar(), b1.boz(), b2.bar());

    // trait bounds
    pub fn both<T: Foo>(item: T) {
        println!("Both methods: {}, {}", item.bar(), item.boz());
    }
    both(a1);
}

fn conditional_trait_methods () {
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self {
                x,
                y,
            }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("x is larger: {}", self.x);
            } else {
                println!("y is larger: {}", self.y);
            }
        }
    }
    let p1 = Pair { x: 1, y: 2 };
    let p2 = Pair { x: 2, y: 1 };
    p1.cmp_display();
    p2.cmp_display();
}

fn lifetimes () {
    // let r;
    // {
    //     let x = 50;
    //     r = &x;
    // }
    // println!("r: {}", r);
    let string1 = String::from("abcd");
    let result;
    {
        // // This works - string2 as slice
        let string2 = "foo";
        result = longest(string1.as_str(), string2);

        // This results in a lifetime error
        // let string2 = String::from("foo");
        // result = longest(string1.as_str(), string2.as_str());
    }
    println!("Longest: {}", result);

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    struct Foo<'a> {
        bar: &'a str,
    }
    let string = String::from("Some random string");
    let first_word = string.split(' ').next().expect("Should have a space");
    let x = Foo { bar: first_word };

    /*
     * Lifetime Elision Rules
     * 1. each reference param gets it's own lifetime param
     * 2. if there is only one input lifetime param, it becomes the lifetime of all outputs
     * 3. if multiple input lifetime params, but one is `self` or `&mut self`, lifetime of `self` becomes the lifetime of all outputs
     */
}
