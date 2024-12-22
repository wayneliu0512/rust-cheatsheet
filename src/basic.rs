// primitive type: Demonstrate how primitive types are modified
pub fn primitive_type() {
    let mut x = 1;

    // modify by assignment
    dbg!(x = x + 1);

    // modify by shadowing
    let mut x = x + 1;
    dbg!(x);

    // modify by reference
    modify_by_ref(&mut x);
    dbg!(x);

    // are copied
    let y = x;
    dbg!(x, y);
    x = 2;
    dbg!(x, y);
    modify_by_move(x);
    dbg!(x, y);
}

fn modify_by_ref(x: &mut i32) {
    *x = *x + 1;
}

fn modify_by_move(mut x: i32) {
    x = x + 1;
    dbg!(x);
}

// compound type: Demonstrate how compound types are modified
#[derive(Debug)]
struct Foo {
    x: i32,
}

pub fn compound_type() {
    let mut foo = Foo { x: 1 };
    dbg!(foo);

    // modify by assignment
    foo = Foo { x: 2 };
    dbg!(foo);

    // modify by shadowing
    let mut foo = Foo { x: 3 };
    foo = dbg!(foo);

    // modify by reference
    modify_by_ref_(&mut foo);
    foo = dbg!(foo);

    // modify by move
    let foo = modify_by_move_(foo);
    dbg!(foo);

    // are move
    // Following line will not compile
    // let bar = foo;
    // println!("foo.x: {}, bar.x: {}", foo.x, bar.x);
}

fn modify_by_ref_(foo: &mut Foo) {
    foo.x = foo.x + 1;
}

fn modify_by_move_(mut foo: Foo) -> Foo {
    foo.x = foo.x + 1;
    foo
}

// struct type: Demonstrate how to use struct type
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    fn display(&self) {
        println!("Point: x={}, y={}", self.x, self.y);
    }
}

// tuple struct
#[derive(Debug)]
struct Point3D(i32, i32, i32);
impl Point3D {
    fn display(&self) {
        println!("Point3D: x={}, y={}, z={}", self.0, self.1, self.2);
    }
}

// unit struct
#[derive(Debug)]
struct Unit;

pub fn struct_() {
    let mut point1 = Point { x: 1, y: 2 };
    point1 = dbg!(point1);

    let point2 = Point { x: 3, ..point1 };
    dbg!(point2);

    let mut point3 = Point::new(4, 5);
    point3.set_x(6);
    point3.display();

    let point4 = Point3D(7, 8, 9);
    point4.display();
    dbg!(point4);

    let unit = Unit;
    dbg!(unit);
}

// enum type: Demonstrate how to use enum type
#[derive(Debug)]
enum IpAddr {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub fn enum_() {
    let mut ip = IpAddr::V4;
    dbg!(ip);
    ip = IpAddr::V6;
    dbg!(ip);

    let mut ip2 = IpAddr2::V6(String::from("::1"));
    match_ip(ip2);
    ip2 = IpAddr2::V4(127, 0, 0, 1);
    match_ip(ip2);

    match_();
}

fn match_ip(ip: IpAddr2) {
    match ip {
        IpAddr2::V4(a, b, c, d) => {
            println!("V4: {a}.{b}.{c}.{d}");
        }
        IpAddr2::V6(s) => {
            println!("V6: {s}");
        }
    }
}

fn match_() {
    let x = 6;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("other"),
    }

    // if let
    let z = Some(3);
    if let Some(3) = z {
        println!("three");
    }
    // Same as above
    match z {
        Some(3) => println!("three"),
        _ => (),
    }
}
