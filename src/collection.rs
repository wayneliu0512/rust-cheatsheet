pub fn vector_() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    v = dbg!(v);

    let third = v.get(3);
    if let Some(value) = third {
        println!("v[3] is: {value}");
    }

    let fourth = &v[3];
    println!("v[3] is: {fourth}");

    for i in &v {
        println!("{i}");
    }
}

use unicode_segmentation::UnicodeSegmentation;
pub fn string_() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    dbg!(s);

    let s1 = String::from("hello");
    let s2 = String::from(", world!");
    let s3 = s1 + &s2; // s1 is moved here and can no longer be used
    dbg!(s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); // s1, s2, s3 are not moved here
    dbg!(s);

    println!("Print in bytes:");
    for b in "नमस्ते".bytes() {
        dbg!(b);
    }

    println!("Print in char:");
    for c in "नमस्ते".chars() {
        dbg!(c);
    }

    println!("Print in grapheme cluster:");
    for c in UnicodeSegmentation::graphemes("नमस्ते", true) {
        dbg!(c);
    }
}

use std::collections::HashMap;
pub fn hash_map_() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50); 
    scores = dbg!(scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    dbg!(score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);
    dbg!(scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    dbg!(map);
}
