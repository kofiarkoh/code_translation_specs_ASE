use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse three long integers.
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
        // Find index of maximum value (first occurrence), then set it to 1.
        let mut m: usize = 0;
        let mut max_val: i64 = 0; // Mirrors the C code's initialization.
        for i in 0..3 {
            if max_val < x[i] {
                max_val = x[i];
                m = i;
            }
        }
        x[m] = 1;

        // Match C's printf("%ld", ...) behavior: no trailing newline.
        print!("{}", x[0] * x[1] * x[2]);
    }
}
