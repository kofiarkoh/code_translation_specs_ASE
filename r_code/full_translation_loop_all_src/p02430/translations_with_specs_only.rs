use std::io::{self, Read};

fn main() {
    // Read stdin; if reading fails, exit with non-zero error code.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        std::process::exit(1);
    }

    let mut it = input.split_whitespace();
    let n: usize = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let k: usize = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };

    // If invalid (k > n), produce no output.
    if k > n {
        return;
    }

    // Enumerate all bit patterns of length n (as an integer mask) in increasing numeric order.
    // Print those with exactly k set bits.
    //
    // The expected output format (per sample) is:
    //   <mask_as_decimal>: <indices_of_set_bits_in_ascending_order>
    //
    // Indices correspond to bit positions: index i is set if (mask >> i) & 1 == 1.
    // This matches the sample where n=4,k=2 yields masks 3,5,6,9,10,12.
    if n >= usize::BITS as usize {
        // Narrowest reasonable interpretation: avoid shifting by >= word size.
        // The spec mentions buffer limits; for too-large n, do nothing rather than panic/UB.
        return;
    }

    let limit: u64 = 1u64 << n;
    let mut out = String::new();

    for mask in 0u64..limit {
        if mask.count_ones() as usize != k {
            continue;
        }

        out.push_str(&mask.to_string());
        out.push(':');

        for i in 0..n {
            if ((mask >> i) & 1) != 0 {
                out.push(' ');
                out.push_str(&i.to_string());
            }
        }
        out.push('\n');
    }

    print!("{out}");
}
