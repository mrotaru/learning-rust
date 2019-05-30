fn main() {
    let x = vec![1, 2, 3];

    // x is moved into the closure when the closure is created,
    // so `x` cannot be used after the closure
    let equal_to_x = move |z| z == x;

    // using it here; won't compile
    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}
