mod col;

fn main() {
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
