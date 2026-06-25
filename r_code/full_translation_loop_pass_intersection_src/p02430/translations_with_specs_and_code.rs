use std::io::{self, Read};

fn main() {
    // Read entire stdin (spec assumes two integers are provided; otherwise behavior is unspecified).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let mut it = input.split_whitespace();
    let n: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let k: u32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);

    // Follow spec/C assumption: n is non-negative and small enough that (1<<n) is well-defined/feasible.
    // Use u32 for bit operations to mirror C int popcount behavior on non-negative patterns.
    let limit: u32 = if n >= 0 && n < 32 { 1u32 << (n as u32) } else { 0 };

    let mut out = String::new();
    for pat in 0u32..limit {
        if pat.count_ones() != k {
            continue;
        }
        out.push_str(&format!("{}:", pat));
        for i in 0..(n as u32) {
            if (pat & (1u32 << i)) != 0 {
                out.push_str(&format!(" {}", i));
            }
        }
        out.push('\n');
    }

    print!("{out}");
}
