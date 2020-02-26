struct Point {
    x: i32,
    y: i32,
}

fn destructuring_structs () {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p; // source first - [source]: [destination]
    println!("a = {}, b = {}", a, b);
    let Point { x, y } = p; // compact syntax
    println!("x = {}, y = {}", x, y);
}

fn destructuring_structs_literals (p: &Point) {
    match p {

        // the following syntax is "experimental":
        // Point { x, y: 0..=5 | -5..=0 } => println!("Point close to x axis (x = {}, y = {}, <= 5)", x, p.y), // https://github.com/rust-lang/rust/issues/54883

        Point { x, y: 0..=5 } => println!("Point close to x axis (x = {}, y = {}, <= 5)", x, p.y),
        Point { x: 0, y } => println!("Point on y axis at {}", y),
        Point { x, y } => println!("Point not close to x axis and not on y: {}, {}", x, y),
    }
}

fn destructuring_enums () {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("Quit; has no data");
        },
        Message::Move { x, y } => { // struct
            println!("Move; x = {}, y = {}", x, y);
        },
        Message::Write(text) => { // single value
            println!("Write; text: {}", text);
        },
        Message::ChangeColor(r, g, b) => { // tuple
            println!("ChangeColor; r = {}, g = {}, b = {}", r, g, b);
        }
    }
}

fn destructuring_references () {
    let points = vec![
        Point { x: 0, y: 1 },
        Point { x: 0, y: 2 },
        Point { x: 0, y: 3 },
    ];

    let sum_y: i32 = points
        .iter()
        // .map(|&Point { x, y }| x * x + y * y)
        // .map(|&Point { y }| y) // invalid - must also match `x`
        .map(|&Point { x: _x, y }| y)
        // .map(|&Point { x: _, y }| y) // equivalent to the above
        .sum();
    println!("Sum of y's: {}", sum_y);
}

fn destructuring_nested () {
    let ((a, b), Point { x, y }) = ((3, 10), Point { x: 1, y: 2 });
    println!("a, b, x, y: {}, {}, {}, {}", a, b, x, y);
}

// ignoring params - sometimes useful with traits
fn params_are_patterns (_: i32, y: i32) {
    println!("Ignoring the first param, second one is {}", y);
}

fn setting_example () {
    let mut existing_value = Some(5);
    let new_value = Some(10);
    match (existing_value, new_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite");
        },
        _ => {
            existing_value = new_value;
        }
    }
    println!("Setting is {:?}", existing_value);
}

fn ignore_unused () {
    let s = Some(String::from("foo"));
    if let Some(_s) = s { // `_s` will only be bound inside this scope
        println!("matched a string: {:?}", _s);
    }
    // println!("s: {:?}", s); // error: borrow of moved value (`s` was moved into `_s`)
    // println!("_s: {:?}", _s); // error: cannot find `_s` in scope

    // if let Some(_) = s { // error: use of moved value (`s`) (value used here after partial move)
        // println!("matched a string: {:?}", _); // error: expected expression
    // }
    let s = Some(String::from("foo")); // shadow previous `s`
    if let Some(_) = s {
        // println!("matched a string: {:?}", _); // error: expected expression
        println!("matched a string."); // error: expected expression
    }
    println!("second s: {:?}", s); // => Some("foo")
}

fn ignore_part () {
    struct Point3D {
        x: i32, y: i32, z: i32,
    }
    let p = Point3D { x: 0, y: 1, z: 2 };
    match p {
        Point3D { y, .. } => println!("I only care about y, ignoring x and z: {}", y),
    }

    let numbers = (1, 2, 3, 4, 5);
    match numbers {
        (first, .., last) => {
            println!("First and last: {}, {}", first, last);
        },
    }
    match numbers {
        (first, ..) => {
            println!("First: {}", first);
        },
        (.., last) => { // unreachable ! (first pattern matches, doesn't get to here)
            println!("Last: {}", last);
        },
    }

    match numbers {
        (.., last) => {
            println!("Last: {}", last);
        },
    }
}

fn reference_patterns () {
    let foo = Some(String::from("foo"));
    match foo {
        Some(value) => println!("got a value: {}", value),
        None => (),
    }
    // println!("using foo: {:?}", foo); // error: borrow of moved value

    // ref keyword has to be used, because `&` _matches_ on a reference, doesn't _create_ one
    let foo = Some(String::from("foo"));
    match foo {
        Some(ref value) => println!("got a value: {}", value),
        None => (),
    }
    println!("using foo: {:?}", foo);

    // ref mut
    let mut foo = Some(String::from("foo"));
    match foo {
        Some(ref mut value) => *value = String::from("bar"),
        None => (),
    }
    println!("using foo: {:?}", foo);
}

fn match_guards (n: Option<i32>) {

    // match n { // error: non-exhaustive pattern (because of the match guard)
    //     Some(x) if x < 5 => println!(">5"),
    //     None => (),
    // }

    match n {
        Some(x) if x < 5 => println!("value, <5: {}", x),
        Some(x) => println!("value, >=5: {}", x),
        None => (),
    }
}

fn matching_guards_shadow () {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50!"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("At the end x = {:?}, y = {:?}", x, y);
}

fn match_guard_if () {
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"), // ≡ (4 | 5 | 6) and y
        _ => println!("no"),
    }
}

fn at_bindings () {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        // @ is like saying "whatever was matched, capture it in ..."
        // in this case: "whatever was matched, capture it in `id_variable`"
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("got id in range: {}", id_variable);
        },
        Message::Hello { id: 10..=12 } => {
            println!("got id in another range, but don't care about the details");
        },
        Message::Hello { id } => {
            println!("not in any of the previous ranges: {}", id);
        }
    }
}

fn main() {
    destructuring_structs();

    let px = Point { x: 12, y: 0 };
    destructuring_structs_literals(&px);

    let py = Point { x: 0, y: 12 };
    destructuring_structs_literals(&py);

    let pxy = Point { x: 12, y: 12 };
    destructuring_structs_literals(&pxy);

    destructuring_enums();

    destructuring_references();

    destructuring_nested();

    params_are_patterns(100, 200);

    setting_example();

    ignore_unused();

    ignore_part();

    reference_patterns();

    match_guards(Some(4));
    match_guards(Some(5));

    matching_guards_shadow();

    match_guard_if();

    at_bindings();
}
