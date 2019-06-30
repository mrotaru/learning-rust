use std::sync::{Mutex, Arc};
use std::thread;
use std::rc::Rc;

fn hello_mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("hello mutex: {:?}", m);
}

// won't work - closure takes ownership on first iteration
fn shared_mutex_broken() {
    let counter = Mutex::new(0);
    // let mut handles = vec![];

    // We have a loop here; and the closure takes ownership of values used
    // inside it. We have one closure per iteration; this means that the first
    // closure will take ownership of `counter`, and the following ones are
    // unable to use it - resulting in an error. Commenting out the whole thing,
    // because Rust will not compile such code, even if it's not invoked.
    // for _ in 0..10 {
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    println!("Result: {}", *counter.lock().unwrap());
}

// won't work - Rc is not thread-safe
fn shared_mutex_also_broken() {
    // let counter = Rc::new(Mutex::new(0));
    // let mut handles = vec![];

    // for _ in 0..10 {
    //     let counter = Rc::clone(&counter);

    //     // Fails with E0277 - required trait not implemented. The unimplemented
    //     // trait is `std::marker::Send`:
    //     // https://doc.rust-lang.org/std/marker/trait.Send.html "Types that can
    //     // be transferred across thread boundaries." Each thread gets a clone,
    //     // which works around the previous issue - but, all clones point to the
    //     // same underlying value; it is a pointer, after all. This is not safe,
    //     // because Rc doesn't use atomic operations.
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("Result: {}", *counter.lock().unwrap());
}

fn shared_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Shared mutex result: {}", *counter.lock().unwrap());
}

fn main() {
    hello_mutex();
    shared_mutex();
}
