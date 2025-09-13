fn fib(n: u32) -> u32 {
    if n < 2 {
        // The base case.
        return n;
    } else {
        // The recursive case.
        return fib(n - 2) + fib(n - 1);
    }
}

/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> i32 {
    let mut length: i32 = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        length += 1;
    }
    length
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed: [[i32; 3]; 3] = [[0; 3]; 3];
    for row in 0..3 {
        for col in 0..3 {
            transposed[col][row] = matrix[row][col]
        }
    }
    transposed
}

fn types_and_values() {
    // Types and Values
    let n = 20;
    println!("fib({n}) = {}", fib(n));
    dbg!(n); // <-- this is ice cream
}

fn control_flow() {
    // Control-flow
    for x in 1..=5 {
        // <-- inclusive range
        dbg!(x);
    }
    println!("Length: {}", collatz_length(11)); // should be 15
}

fn arrays() {
    // Arrays
    let mut a: [i8; 5] = [42; 5];
    a[0] = 66;
    assert_eq!(a, [66, 42, 42, 42, 42]);
    dbg!(a);
    println!("a: {:?}", a);
    println!("a: {:#?}", a); // < -- pretty printing
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0)
        }
    }
}

fn tuples() {
    //Tuples
    let unit_type = ();
    assert_eq!(unit_type, ());

    let mut b: (i8, bool) = (8, false);
    b.0 = 42;
    assert_eq!(b.0, 42);
    assert_eq!(b.1, false);
}

fn patterns_and_destructuring() {
    //Patterns & Destructuring
    let tuple = (1, 5, 3);
    let (left, middle, right) = tuple;
    let order = if left < middle && middle < right {
        "ordered"
    } else {
        "unordered"
    };
    assert_eq!(order, "unordered");

    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    dbg!(matrix);
    let transposed = transpose(matrix);
    dbg!(transposed);
}

fn shared_references() {
    //Shared references
    let a = 'A';
    let b = 'B';

    let mut r: &char = &a;
    dbg!(r);

    r = &b;
    dbg!(r);
}

fn exclusive_references() {
    //Exclusive references
    let mut c = 'C'; // shared reference that can be bound to different values
    let z = &mut c; // exclusive reference to a mutable value
    *z = 'd';
    dbg!(z);
    dbg!(c);
}

fn reference_validity() {
    //Reference validity
    // let x_ref = {
    //     let x = 10;
    //     &x // error > x does not outlive x_ref
    // };
    // dbg!(x_ref);
}

fn slices() {
    //Slices
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];

    println!("s: {s:?}");
}

fn strings() {
    //Strings
    let s1: &str = "hello"; // &[u8], string slice
    let s2: String = String::from("world"); // Vec<T>
    dbg!(s1);
    dbg!(s2);
}

fn exercise() {
    //Exercise
    println!(
        "Magnitude of a unit vector: {}",
        magnitude(&[0.0, 1.0, 0.0])
    );

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}

// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.

fn magnitude(v: &[f64; 3]) -> f64 {
    let squared_stum: f64 = v.iter().map(|x| return x * x).sum();
    squared_stum.sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.

fn normalize(v: &mut [f64; 3]) {
    let m = magnitude(v);
    let _ = v.iter_mut().map(|x| return *x / m);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn named_structs() {
    let mut peter = Person {
        name: String::from("Peter"),
        age: 28,
    };
    let avery = Person {
        name: String::from("Avery"),
        age: 10,
    };
    let jackie = Person {
        name: String::from("Jackie"),
        ..avery
    };
    peter.age = 38;
    dbg!(peter.name);
    dbg!(peter.age);
    dbg!(avery);
    dbg!(jackie);
}

#[derive(Debug)]
struct Point(i32, i32);

//Single-field wrappers a.k.a NewTypes
#[derive(Debug)]
struct Newtons(f64);

fn tuple_structs() {
    let p = Point(12, 13);
    dbg!(p.0);
    dbg!(p.1);
    let n = Newtons(10.0);
    println!("{:?}", n.0);
}

pub fn day1() {
    types_and_values();
    control_flow();
    arrays();
    tuples();
    patterns_and_destructuring();
    shared_references();
    exclusive_references();
    reference_validity();
    slices();
    strings();
    exercise();
    named_structs();
    tuple_structs();
}
