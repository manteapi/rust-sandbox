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

fn main() {
    // Types and Values
    let n = 20;
    println!("fib({n}) = {}", fib(n));
    dbg!(n); // <-- this is ice cream

    // Control-flow
    for x in 1..=5 {
        // <-- inclusive range
        dbg!(x);
    }
    println!("Length: {}", collatz_length(11)); // should be 15

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

    //Tuples
    let unit_type = ();
    assert_eq!(unit_type, ());

    let mut b: (i8, bool) = (8, false);
    b.0 = 42;
    assert_eq!(b.0, 42);
    assert_eq!(b.1, false);

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
