// match Arms
/*
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}

let PATTERN = EXPRESSION;
*/

pub fn pattern() {
    pattern_in_while();
    pattern_in_for();
    pattern_in_function();
    multiple_patterns();
    destructuring_struct();
    destructuring_enum();
    destructuring_struct_tuple();
}

fn pattern_in_while() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }
}

fn pattern_in_for() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{index}: {value}");
    }
}

fn pattern_in_function() {
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("{x}, {y}");
}

fn multiple_patterns() {
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        1..=5 => println!("one through five"),
        _ => println!("anything"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn destructuring_struct() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    println!("{a}, {b}");

    let Point { x, y } = p;
    println!("{x}, {y}");

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}

fn destructuring_enum() {
    enum Color {
        Rgb(i32, i32, i32),
        _Hsv(i32, i32, i32),
    }
    enum Message {
        _Quit,
        _Move { x: i32, y: i32 },
        _Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));

    match msg {
        Message::_Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::_Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::_Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::_Hsv(h, s, v)) => {
            println!("Change the color to hue {h}, saturation {s}, and value {v}");
        }
    }
}

fn destructuring_struct_tuple() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("{feet}, {inches}, {x}, {y}");
}

pub fn ignore() {
    ignore_variable();
    ignore_part_of_variable();
    ignore_unused_variable();
    ignore_remaining_part_of_variable();
}

fn ignore_variable() {
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {y}");
    }

    foo(3, 4);
}

fn ignore_part_of_variable() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
}

fn ignore_unused_variable() {
    let _x = 5;
    let y = 10;
    let _z = 15;

    println!("{y}");
}

fn ignore_remaining_part_of_variable() {
    struct Point {
        x: i32,
        _y: i32,
        _z: i32,
    }

    let origin = Point { x: 0, _y: 0, _z: 0 };

    match origin {
        Point { x, .. } => println!("{x}"),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}

pub fn extra_condition() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {x:?}, y = {y}");
}

pub fn binding() {
    let x = 1;
    match x {
        e @ 1..=5 => println!("Got a range element {e}"),
        _ => println!("Anything"),
    }
}

