use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let mut it = input.split_whitespace();

    let bit_width: i64 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let required: i64 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);

    if bit_width < 0 || required < 0 {
        return;
    }

    let bw = bit_width as u32;
    if bw >= 128 {
        return;
    }

    let limit: u128 = 1u128 << bw;

    let mut out = String::new();

    for x in 0u128..limit {
        let mut count: i64 = 0;
        for i in 0..bw {
            if ((x >> i) & 1) != 0 {
                count += 1;
            }
        }

        if count == required {
            // Match required output format: "<value>: <idx0> <idx1> ..."
            out.push_str(&x.to_string());
            out.push(':');
            for i in 0..bw {
                if ((x >> i) & 1) != 0 {
                    out.push(' ');
                    out.push_str(&i.to_string());
                }
            }
            out.push('\n');
        }
    }

    print!("{out}");
}
