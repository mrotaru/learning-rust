fn main() {
    // Book (1st ed) says the below is not valid because it's possible that T is a ref 
    // containing other refs to stuff that is not guaranteed to outlive 'a - so we'd have to explicitly
    // add a lifetime, to indicate this constraint - that T and it's (referenced) contents must not live
    // longer than 'a. However, in 1.43 it's fine - compiler detects when 
    // struct Ref<'a, T: 'a>(&'a T); // <-- required previously

    struct Ref<'a, T>(&'a T); // <-- this works in 1.43

    let x = 100;
    let ref_x = Ref(&x);
    println!("ref_x: {:?}", *ref_x.0);

    // Such cases are detected by the compiler automatically.
    // For the code below, we'll get "y does not live long enough"
    // let ref_y;
    // {
    //     let y:i32 = 200;
    //     ref_y = Ref(&y);
    // }
    // println!("ref_y: {:?}", *ref_y.0);
}
