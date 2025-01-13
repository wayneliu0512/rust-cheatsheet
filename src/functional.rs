// Closures can capture values from their environment in three ways, 
// which directly map to the three ways a function can take a parameter: 
// borrowing immutably, borrowing mutably, and taking ownership. 
// The closure will decide which of these to use based on what the body of
// the function does with the captured values.
use std::thread;
pub fn closure() {
    let mut list = vec![1, 2, 3];

    let only_borrows = || println!("From closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("After calling closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
    .join()
    .unwrap();
}

// The Fn, FnMut, and FnOnce traits are implemented by closures to
// allow them to be called. The Fn trait is implemented for closures
// that borrow values from their environment immutably. The FnMut trait
// is implemented for closures that borrow values from their environment
// mutably. The FnOnce trait is implemented for closures that take ownership
// of values from their environment.
pub fn fn_trait() {

}