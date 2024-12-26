pub fn function() {
    let number_list = vec![34, 50, 25, 100, 65];
    dbg!(largest(&number_list));
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn struct_() {
    let integer_and_float = Point { x: 5, y: 4.0 };
    println!("x:{}, y:{}", integer_and_float.x, integer_and_float.y);

    let string_and_char = Point { x: "Hello", y: 'c' };
    println!("x:{}, y:{}", string_and_char.x, string_and_char.y);
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

pub fn multi_generic_types() {
    let p1 = Point_ { x: 5, y: 4.0 };
    let p2 = Point_ { x: "Hello", y: 'c' };
    let p3 = p1.mixyup(p2);
    dbg!(p3);
}

#[derive(Debug)]
struct Point_<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point_<X1, Y1> {
    fn mixyup<X2, Y2>(self, other: Point_<X2, Y2>) -> Point_<X1, Y2> {
        Point_ {
            x: self.x,
            y: other.y,
        }
    }
}