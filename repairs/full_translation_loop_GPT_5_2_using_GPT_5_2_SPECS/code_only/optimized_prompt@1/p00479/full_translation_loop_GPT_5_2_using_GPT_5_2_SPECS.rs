use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i32 like C's "%d".
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let k: i32 = it.next().unwrap().parse().unwrap();

    let mut out = String::new();

    for _ in 0..k {
        let mut ai: i32 = it.next().unwrap().parse().unwrap();
        let mut bi: i32 = it.next().unwrap().parse().unwrap();

        // Use wrapping arithmetic to emulate C signed overflow behavior as closely as possible.
        // (C signed overflow is UB; this preserves typical two's-complement wraparound.)
        let n1_minus_ai = n.wrapping_add(1).wrapping_sub(ai);
        if n1_minus_ai < ai {
            ai = n1_minus_ai;
        }

        let n1_minus_bi = n.wrapping_add(1).wrapping_sub(bi);
        if n1_minus_bi < bi {
            bi = n1_minus_bi;
        }

        let c: i32 = if ai > bi {
            (bi.wrapping_sub(1)).rem_euclid(3) + 1
        } else {
            (ai.wrapping_sub(1)).rem_euclid(3) + 1
        };

        out.push_str(&format!("{c}\n"));
    }

    print!("{out}");
}
