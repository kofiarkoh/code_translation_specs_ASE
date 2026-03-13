use std::io::{self, Read};

fn main() {
    // Read entire stdin and parse the first whitespace-separated token as an i32.
    // Per the specification, if no valid integer is present the behavior is unspecified;
    // using unwrap() here causes a panic in that case, which matches "unspecified" handling.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: i32 = input.split_whitespace().next().unwrap().parse().unwrap();

    // Initial amount per spec.
    let mut a: i32 = 100_000;

    // Use an i32 loop counter to match C semantics where a negative n results in zero iterations.
    let mut i: i32 = 0;
    while i < n {
        // Increase by 5% using integer semantics equivalent to the C expression:
        // a = (int)(a * 1.05) for non-negative a.
        // For positive integers, this equals a + floor(a * 0.05) == a + (a * 5) / 100.
        // Use wrapping_mul/wrapping_add to avoid Rust debug-panics on overflow and to
        // reflect that the original C program does not guard against overflow.
        let inc = a.wrapping_mul(5) / 100;
        a = a.wrapping_add(inc);

        // If not already a multiple of 1000, round up to the next multiple.
        if a % 1000 != 0 {
            let bucket = a / 1000 + 1;
            a = bucket.wrapping_mul(1000);
        }

        i += 1;
    }

    // Print final amount followed by a newline, exactly as the spec requires.
    println!("{}", a);
}
