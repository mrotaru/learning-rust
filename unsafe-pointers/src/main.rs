fn main() {
    let mut num = 5;

    // raw pointers can be created freely, that is safe ...
    // NOTE: both pointers point to the same location - but one is mutable, and
    // the other one is immutable. This would not be allowed if we were using
    // safe (standard) references.
    let r1 = &num as *const i32; // immutable raw pointer
    let r2 = &mut num as *mut i32; // mutable raw pointer

    // ... but dereferencing raw pointers is not safe, must be enclosed within
    // an `unsafe` block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe fn dangerous () {
        println!("I'm not really dangerous");
    }

    // without `unsafe` block, compiler error
    // call to unsafe function is unsafe and requires unsafe function or block
    // dangerous();
    unsafe {
        dangerous();
    }

    // Re-implement std `split_at_mut`
    // https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_mut
    // Divides one mutable slice into two at an index.
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let slice = &mut v[..];
    let (a, b) = slice.split_at_mut(3);
    println!("Slice a: {:?}", a);
    println!("Slice b: {:?}", b);

    fn my_split_at_mut(slice: &mut [i32], index: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        assert!(index <= len);
        // cannot borrow as mutable multiple times;
        // first borrowed for first slice, then for the second one
        // (&mut slice[..index], &mut slice[index..])

        // so, even though we're sure the code is fine, Rust won't let us run it.
        // For such cases, we can use raw pointers and `unsafe`
        let ptr = slice.as_mut_ptr(); // **creating** raw pointers is safe
        unsafe { // .. but using them, is not
                (
                    std::slice::from_raw_parts_mut(ptr, index),
                    std::slice::from_raw_parts_mut(ptr.offset(index as isize), len - index)
            )
        }
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let slice = &mut v[..];
    let (a, b) = my_split_at_mut(slice, 3);
    println!("Using the unsafe function:");
    println!("Slice a: {:?}", a);
    println!("Slice b: {:?}", b);
}
