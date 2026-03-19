use std::io::{self, Read};

fn expected_value(pos: i64, p: f64) -> f64 {
    if pos % 2 == 0 {
        // Even: E(pos) = (pos/2)/p
        // Use floating division; if p == 0.0 this yields +/-inf or NaN per IEEE-754.
        (pos as f64 / 2.0) / p
    } else {
        // Odd: E(pos) = 1 + E(pos-1)*p + E(pos+1)*(1-p)
        // Recursive as specified; may not terminate for some inputs.
        1.0 + expected_value(pos - 1, p) * p + expected_value(pos + 1, p) * (1.0 - p)
    }
}

fn main() {
    // Specification describes both normal and test mode, but provides no mechanism
    // to select test mode or file paths. Narrowest interpretation: implement normal mode.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let pos: i64 = it.next().unwrap().parse().unwrap();
    let percent: f64 = it.next().unwrap().parse().unwrap();
    let p = percent / 100.0;

    let ans = expected_value(pos, p);

    // Fixed precision, deterministic formatting.
    // The exact number of decimals is not specified; choose a conventional fixed precision.
    println!("{:.10}", ans);
}
