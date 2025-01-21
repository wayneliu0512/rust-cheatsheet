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

pub fn shared_state() {
    mutex();
    arc();
}

use std::sync::{Mutex, Arc};
fn mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
}

// `Arc<T>` is a type like `Rc<T>`` that is safe to use in concurrent
// situations. The a stands for atomic, meaning it’s an atomically reference
// counted type
fn arc() {
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

    println!("Result: {}", *counter.lock().unwrap());
}


// `Sync` and `Send` traits
//
// The `Send` marker trait indicates that ownership of values of the type 
// implementing `Send` can be transferred between threads. Almost every
// Rust type is `Send`. Any type composed entirely of `Send` types is 
// automatically marked as `Send` as well.
//
// The `Sync` marker trait indicates that it is safe for the type implementing
// `Sync` to be referenced from multiple threads. In other words, any type `T`
// is `Sync` if `&T` (a reference to `T`) is `Send`, meaning the reference can
// be sent safely to another thread. Similar to `Send`, primitive types are
// `Sync`, and types composed entirely of types that are `Sync` are also `Sync`.