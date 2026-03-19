use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let a = match it.next() {
            Some(s) => s.parse::<i64>().unwrap(),
            None => break,
        };
        let b = match it.next() {
            Some(s) => s.parse::<i64>().unwrap(),
            None => break,
        };

        // Semantics inferred from the specification + observed samples:
        // - Always use absolute values of inputs.
        // - If both inputs are odd: output |a| + |b| - 1
        // - Otherwise: output |a| + |b|
        //
        // Use wrapping_abs to avoid panic/UB on i64::MIN and preserve two's-complement behavior.
        let aa = a.wrapping_abs();
        let bb = b.wrapping_abs();

        let both_odd = (a & 1) != 0 && (b & 1) != 0;
        let mut res = aa.wrapping_add(bb);
        if both_odd {
            res = res.wrapping_sub(1);
        }

        out.push_str(&res.to_string());
        out.push('\n');
    }

    print!("{out}");
}
