use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let h: i32 = match it.next() {
            Some(t) => t.parse().unwrap(),
            None => break,
        };
        let m: i32 = match it.next() {
            Some(t) => t.parse().unwrap(),
            None => break,
        };
        let s: i32 = match it.next() {
            Some(t) => t.parse().unwrap(),
            None => break,
        };

        if h == -1 && m == -1 && s == -1 {
            break;
        }

        // In C, signed overflow is undefined; typical behavior is two's-complement wrap.
        // Use wrapping arithmetic to emulate that common behavior deterministically.
        let cur_sec = h
            .wrapping_mul(3600)
            .wrapping_add(m.wrapping_mul(60))
            .wrapping_add(s);

        let mut remaining = (2i32).wrapping_mul(3600).wrapping_sub(cur_sec);

        // C integer division/modulo truncates toward zero; Rust i32 does the same.
        println!(
            "{:02}:{:02}:{:02}",
            remaining / 3600,
            (remaining % 3600) / 60,
            remaining % 60
        );

        remaining = remaining.wrapping_mul(3);

        println!(
            "{:02}:{:02}:{:02}",
            remaining / 3600,
            (remaining % 3600) / 60,
            remaining % 60
        );
    }
}
