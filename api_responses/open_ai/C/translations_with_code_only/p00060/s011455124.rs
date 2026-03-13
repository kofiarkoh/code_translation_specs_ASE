use std::io::{self, Read};

fn main() {
    // Read all stdin into a string, then simulate scanf("%d %d %d") semantics:
    // sequentially parse whitespace-separated tokens as integers and stop on
    // EOF or on the first parse failure (like scanf would).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");

    // Initialize card array as in the C code (though it's unused afterwards).
    let mut card = [0i32; 11];
    for i in 1..11 {
        card[i] = 1;
    }

    let mut iter = input.split_whitespace();

    loop {
        // Attempt to read three integers; if any are missing or fail to parse,
        // stop processing (matching scanf behavior).
        let s1 = match iter.next() {
            Some(s) => s,
            None => break,
        };
        let c1: i32 = match s1.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        let s2 = match iter.next() {
            Some(s) => s,
            None => break,
        };
        let c2: i32 = match s2.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        let s3 = match iter.next() {
            Some(s) => s,
            None => break,
        };
        let c3: i32 = match s3.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        let mut cnt = 0;
        // Loop i = 1 .. 10 as in the C code.
        for i in 1..=10 {
            let ii = i as i32;
            if c1 != ii && c2 != ii && c3 != ii {
                // Use wrapping_add to emulate typical two's-complement wraparound
                // behavior of C signed integer overflow on common platforms.
                let sum = c1.wrapping_add(c2).wrapping_add(ii);
                if sum <= 20 {
                    cnt += 1;
                }
            }
        }

        if cnt >= 4 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
