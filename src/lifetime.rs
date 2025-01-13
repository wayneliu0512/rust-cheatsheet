pub fn function() {
    pass();
    // fail();
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn pass() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
}

// fn fail() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {result}");
// }


pub fn struct_() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    dbg!(i.part);
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Lifetime Elision
// Lifetimes on function or method parameters are called input lifetimes,
// and lifetimes on return values are called output lifetimes.
// 
// The first rule applies to input lifetimes, and the second and third rules 
// apply to output lifetimes.
// 
// 1. Each elided lifetime in the parameters becomes a distinct lifetime parameter.
//   fn foo<'a>(x: &'a i32)
//   fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
// 
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to
// all output lifetime parameters.
//   fn foo<'a>(x: &'a i32) -> &'a i32
// 
// 3. If there are multiple input lifetime parameters, but one of them is &self or 
// &mut self, the lifetime of self is assigned to all output lifetime parameters.
//   impl<'a> ImportantExcerpt<'a> {
//       fn announce_and_return_part(&self, announcement: &str) -> &str {
//           println!("Attention please: {announcement}");
//           self.part
//       }
//   }

// The Static Lifetime
pub fn static_() {
    let s: &'static str = "I have a static lifetime.";
    dbg!(s);
}

// Generic Type Parameters, Trait Bounds, and Lifetimes Together
pub fn generic_trait_bound_lifetime() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest_with_an_announcement(string1.as_str(), string2, "Today is a good day");
    println!("The longest string is {result}");
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}