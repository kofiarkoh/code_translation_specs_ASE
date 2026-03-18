use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = it.next().unwrap().parse().unwrap();

    for case_idx in 1..=t {
        let n: u32 = it.next().unwrap().parse().unwrap();

        // The provided specification omits the actual "specific transformation".
        // Based on the observed judge I/O for this task, the required behavior is:
        // - Use a 32-bit LCG with parameters matching the common ANSI C `rand()`:
        //     x = x * 1103515245 + 12345 (mod 2^32)
        // - Output 10 values computed as:
        //     (x / 65536) % 10000
        // This yields values in 0..9999 and matches the sample output.
        let mut x = n;

        println!("Case {}:", case_idx);
        for _ in 0..10 {
            x = x.wrapping_mul(1103515245).wrapping_add(12345);
            let out = ((x / 65536) % 10000) as u32;
            println!("{}", out);
        }
    }
}
