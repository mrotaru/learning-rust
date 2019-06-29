use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn simple_send_rcv() {
    let (tx,rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val: {}", val); // won't work - `val` was moved into `send`
    });

    let received = rx.recv().unwrap(); // this blocks
    // let received = rx.try_recv().unwrap(); // this doesn't block; but will panic, since a value won't be immediately available
    println!("Got: {}", received);
}

fn simple_send_rcv_delayed() {
    let (tx,rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("[spawned thread] 1"),
            String::from("[spawned thread] 2"),
            String::from("[spawned thread] 3"),
            String::from("[spawned thread] 4"),
            String::from("[spawned thread] 5"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // `rx` is a received; but we're not invoking `recv` on it - instead, we're using it
    // as an iterator.
    for received in rx {
        println!("Got: {}", received);
    }
}

fn multiple_producers() {
    let (tx1,rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx1);
    thread::spawn(move || {
        let vals = vec![
            String::from("[spawned thread 1] 1"),
            String::from("[spawned thread 1] 2"),
            String::from("[spawned thread 1] 3"),
            String::from("[spawned thread 1] 4"),
            String::from("[spawned thread 1] 5"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // let tx2 = mpsc::Sender::clone(&tx1); // won't work - tx1 was moved into closure
    thread::spawn(move || {
        let vals = vec![
            String::from("[spawned thread 2] 1"),
            String::from("[spawned thread 2] 2"),
            String::from("[spawned thread 2] 3"),
            String::from("[spawned thread 2] 4"),
            String::from("[spawned thread 2] 5"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn main() {
    // simple_send_rcv();
    // simple_send_rcv_delayed();
    multiple_producers();
}
