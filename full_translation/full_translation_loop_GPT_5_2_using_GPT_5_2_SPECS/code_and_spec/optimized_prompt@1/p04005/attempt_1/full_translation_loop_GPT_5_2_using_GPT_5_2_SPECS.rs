use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse three signed integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut x = [0i64; 3];
    for i in 0..3 {
        x[i] = it.next().unwrap().parse::<i64>().unwrap();
    }

    if x[0] % 2 == 0 || x[1] % 2 == 0 || x[2] % 2 == 0 {
        // Match C's puts("0") behavior: prints "0\n"
        println!("0");
    } else {
        // Find index of a (strictly) largest element; ties keep the first encountered.
        // Note: C code initializes `max` as int 0, which would be wrong for all-negative odds.
        // The SPEC defines "largest of the three values", so we implement that correctly.
        let mut m = 0usize;
        let mut max = x[0];
        for i in 1..3 {
            if x[i] > max {
                max = x[i];
                m = i;
            }
        }

        x[m] = 1;
        // No trailing newline, matching C's printf("%ld", ...)
        print!("{}", x[0] * x[1] * x[2]);
    }
}
