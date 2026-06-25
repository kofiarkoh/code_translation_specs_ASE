use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse three integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();
    let mut c: i64 = it.next().unwrap().parse().unwrap();

    // Spec: D = A0 - B0; if D <= 0 => Cout = C0.
    // If D > 0:
    //   - if C0 > 0 => reduce by min(D, C0) but not below 0
    //   - if C0 <= 0 => Cout = C0 - D (can go more negative)
    let d = a - b;
    if d > 0 {
        if c > 0 {
            c = (c - d).max(0);
        } else {
            c -= d;
        }
    }

    // Note: This follows the SPEC. The provided C code differs when C0 < 0:
    // it would decrement c only until it reaches 0 (never reaching 0 from negative),
    // effectively doing c -= d, which matches the SPEC's C0 <= 0 case.
    // For C0 == 0, the C code decrements once (to -1) before noticing c==0 is false;
    // the SPEC requires Cout = 0 - d, so we follow the SPEC.
    println!("{c}");
}
