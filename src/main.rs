use std::ops::Deref;

mod col;
mod nested;
mod geometry;
mod elevator;
mod evaluate_expression;
mod verbosity_filter;

fn main() {
    elevator::main();

    println!("Hello, world!");
    println!("Wow");

    let x: i32 = 5;
    println!("x: {x}");
    println!("Result of interproduct: {}", interproduct(100, 200, 300));

    let n = 20;
    println!("The {n}th Fibonacci number is {}", fib(n));

    let size = if x < 20 { "small" } else { "large" };
    println!("Size is {size}");

    for i in 1..=5 {
        println!("The {i}th loop iteration");
    }

    let mut i = 0;
    loop {
        i += 1;
        if dbg!(i) > 4 {
            break;
        }
    }

    let y = {
        let z = 10;
        println!("z: {z}");
        z - x
    };

    println!("y: {y}");

    let a = 10;
    println!("before: {a}");

    println!("after: {}", {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
        10
    });

    col::main();

    let array: [i16; 10] = [42; 10];
    println!("array: {array:#?}");

    let tuple: (i8, bool) = (1, true);
    println!("tuple: {tuple:#?}");
    println!("first tuple component: {}", tuple.0);
    println!("second tuple component: {}", tuple.1);

    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23];
    for prime in primes {
        dbg!(prime);
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }

    nested::main();

    let a = 'A';
    let b = 'B';
    let r: &char = &a;
    println!("{}", *r);
    println!("{r}");

    exclusive();
    slices();
    strings();

    // Geometry
    geometry::main();

    structs();
    point();
}

fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    a * b + b * c + c * a
}

fn fib(n: u32) -> u32 {
    if n < 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
fn print_tuple(tuple: (i32, i32)) {
    let (left, right) = tuple;
    println!("left: {left}, right: {right}");
}

fn exclusive() {
    let mut point = (1, 2);
    let x_coord = &mut point.0;
    *x_coord = 20;
    println!("point: {point:#?}");
}

fn slices() {
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..=4];

    println!("s: {s:?}");
}

fn strings() {
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[s2.len() - s1.len()..];
    println!("s3: {s3}");

    println!("{:#?}", b"abc");
    println!("{:#?}", &[97, 98, 99]);

    println!(r#"<a href="link.html">link</a>"#);
    println!("<a href=\"link.html\">link</a>");
}

struct Person {
    name: String,
    age: u8,
    height: f32,
}

fn describe(person: &Person) {
    println!("{} is {} years old and is {} meters long", person.name, person.age, person.height);
}

fn structs() {
    let mut peter = Person { name: String::from("Peter"), age: 27, height: 2.10 };
    describe(&peter);

    peter.age = 28;
    describe(&peter);

    let name = String::from("Avery");
    let age = 39;
    let height = 1.65;
    let avery = Person { name, age, height };
    describe(&avery);

    let jackie = Person { name: String::from("Jackie"), ..avery };
    describe(&jackie);
}

struct Point(i32, i32);

fn point() {
    let p = Point(1, 2);
    println!("({}, {})", p.0, p.1);
}

struct PoundsOfForce(f64);
struct Newtons(f64);

