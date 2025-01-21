pub fn trait_object() {
    println!("Trait Object");
    println!("============");

    // Trait Object
    // ===========
    // Trait object is a way to use trait as a type. It is useful when we want to store different types that implement the same trait in a collection.
    // 
    // In the example below, we have a trait `Animal` with a method `sound`. We have two structs `Dog` and `Cat` that implement the `Animal` trait.
    // We then create a vector of trait objects that store `Dog` and `Cat` instances.
    // 
    // We can call the `sound` method on each element in the vector, even though the vector stores different types.
    // 
    // ```
    // trait Animal {
    //     fn sound(&self);
    // }
    // 
    // struct Dog;
    // impl Animal for Dog {
    //     fn sound(&self) {
    //         println!("Dog barks");
    //     }
    // }
    // 
    // struct Cat;
    // impl Animal for Cat {
    //     fn sound(&self) {
    //         println!("Cat meows");
    //     }
    // }
    // 
    // fn trait_object() {
    //     let dog = Dog;
    //     let cat = Cat;
    // 
    //     let animals: Vec<Box<dyn Animal>> = vec![Box::new(dog), Box::new(cat)];
    // 
    //     for animal in animals {
    //         animal.sound();
    //     }
    // }
    // ```

    trait Animal {
        fn sound(&self);
    }

    struct Dog;
    impl Animal for Dog {
        fn sound(&self) {
            println!("Dog barks");
        }
    }

    struct Cat;
    impl Animal for Cat {
        fn sound(&self) {
            println!("Cat meows");
        }
    }

    let dog = Dog;
    let cat = Cat;

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(dog), Box::new(cat)];

    for animal in animals {
        animal.sound();
    }
}