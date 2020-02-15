use std::thread;
use std::time::Duration;

fn spawned_not_completing () {
    println!("--- 1 ---");
    thread::spawn(|| {
        for i in 1..10 {
            println!("1. [spawned thread] {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("1. [main thread] {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    // The loop in the spawned thread takes about 10ms; the loop in the main
    // thread about 5. As it is here, the main thread will not wait for the
    // spawned thread to complete.
}

fn wait_for_me_please () {
    println!("--- 2 ---");
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("2. [spawned thread] {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // handle.join().unwrap(); // block until the underlying thread terminates
    for i in 1..5 {
        println!("2. [main thread] {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap(); // block until the underlying thread terminates
}

fn move_data () {
    println!("--- 3 ---");
    let value = 42;

    // Can't use values unless closure has `move`. If it does, then values
    // referenced inside the closure are moved into it; the closure will become
    // the owner of the moved values. Can't just use references, even with
    // lifetimes, because the lifetime of a thread is unknown at compile time.
    let handle = thread::spawn(move || {
        println!("value: {}", value);
    });

    // let value = 42; // won't work, because `value` was moved into the closure
    // drop(value); // same here - `value` moved, so can't use it

    handle.join().unwrap();
}

fn main() {
    spawned_not_completing();
    wait_for_me_please();
    move_data();
}
