use std::thread;
use std::time::Duration;

pub fn thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}

use std::sync::mpsc;
pub fn message_passing() {
    single_producer();
    multi_producer();
}

fn single_producer() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    // Non-blocking
    // let received = rx.try_recv().unwrap();
    println!("Got: {received}");
}

fn multi_producer() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let val = String::from("hi");
        tx1.send(val).unwrap();
    });

    thread::spawn(move || {
        let val = String::from("from the other thread");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
