// Smart pointers are usually implemented using structs. Unlike an ordinary
// struct, smart pointers implement the `Deref` and `Drop` traits. The Deref
// trait allows an instance of the smart pointer struct to behave like a
// reference so you can write your code to work with either references or
// smart pointers.

// Most commonly used smart pointers are:
// - `Box<T>` for allocating values on the heap
// - `Rc<T>`, a reference counting type that enables multiple ownership
// - `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that
// enforces the borrowing rules at runtime instead of compile time
pub fn deref() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Deref coercion converts a reference to a type that implements the Deref trait
// into a reference to another type. For example, deref coercion can convert
// `&String` to `&str` because String implements the Deref trait such that it returns
// `&str`.
pub fn deref_coercion() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // If Rust didn't have deref coercion, the code above would have to be written as:
    // hello(&(*m)[..]);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

// How Deref Coercion Interacts with Mutability:
// - From `&T` to `&U` when `T: Deref<Target=U>`
// - From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
// - From `&mut T` to `&U` when `T: Deref<Target=U>`

pub fn drop() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    dbg!(c);
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    dbg!(d);
    println!("CustomSmartPointers created.");
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// Reference counting: `Rc<T>`
use std::rc::Rc;
use List::{Cons, Nil};
pub fn rc() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

// `Box<T>`, `Rc<T>`, `RefCell<T>`:
// - `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>`
//   have single owners.
// - `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>`
//   allows only immutable borrows checked at compile time; `RefCell<T>` allows
//   immutable or mutable borrows checked at runtime.
// - Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate
//   the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.
#[derive(Debug)]
enum ListRefCell {
    ConsR(Rc<RefCell<i32>>, Rc<ListRefCell>),
    NilR,
}

use ListRefCell::{ConsR, NilR};
use std::cell::RefCell;
pub fn ref_cell() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(ConsR(Rc::clone(&value), Rc::new(NilR)));

    let b = ConsR(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = ConsR(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
