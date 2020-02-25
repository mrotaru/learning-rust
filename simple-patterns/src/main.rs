fn separator() {
    println!("-----------------------------------------");
}

fn conditionals () {
    println!("-----------------------------------------");
    // let fauvorite_colour: Option<&str>  = Some("green");
    let fauvorite_colour: Option<&str>  = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    fn set_bg(colour: &str, message: Option<&str>) {
        println!("Setting background to \"{}\" because {}.", colour, message.unwrap_or("why not"));
    }

    if let Some(colour) = fauvorite_colour {
        set_bg(colour, None);
    } else if is_tuesday {
        set_bg("green", Some("tuesday is green day"));
    } else if let Ok(age) = age { // creates a new `age`, shadowing existing one
        if age > 30 { // new `age` variable bound in this new scope
            set_bg("purple", Some("you're old"));
        } else {
            set_bg("orange", Some("you're young"));
        }
    } else {
        set_bg("blue", Some("it's a nice default"));
    }
}

fn while_let () {
    separator();
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("popped: {}", top);
    }
}

fn for_loops () {
    separator();
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn let_is_also_a_pattern () {
    separator();
    let (x, y, z) = (1, 2, 3);
    println!("{}, {}, {}", x, y, z);
}

//               pattern
//                  |
//                  V
//               |-----|
fn param_pattern(&(x, y): &(i32, i32)) {
    separator();
    println!("Params: {}, {}", x, y);
}

fn matching_literals (x: i32) {
    separator();
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("something else, not 1, 2 or 3")
    }
}

fn matching_named_variables () {
    separator();
    let x = Some(5);
    let y = 10;

    match x { // match starts a new scope, so `y` declared above will be shadowed
        Some(50) => println!("Got 50!"),
        Some(y) => println!("Matched, y = {:?}", y), // named variable - irrefutable pattern
        _ => println!("Default case, x = {:?}", x),
    }

    println!("At the end x = {:?}, y = {:?}", x, y);
}

fn match_multiple_patterns (x: i32) {
    separator();
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("..."),
    }
}

fn match_range (ph: i32) {
    separator();

    // note: the code below works, but `...` is deprecated:
    // let acidity = match ph {
    //     0...6 => "acid",
    //     7 => "neutral",
    //     8...14 => "base",
    //     _ => unreachable!(),
    // };

    let acidity = match ph {
        0..=6 => "acid",
        7 => "neutral",
        8..=14 => "base",
        _ => unreachable!(),
    };
    println!("acidity: {}", acidity);
}

fn main() {
    conditionals();
    while_let();
    for_loops();
    let_is_also_a_pattern();

    let point = (100, 200);
    param_pattern(&point);

    matching_literals(2);
    matching_literals(42);

    matching_named_variables();

    match_multiple_patterns(1);
    match_multiple_patterns(2);
    match_multiple_patterns(3);
    
    match_range(6);
    match_range(8);
}
