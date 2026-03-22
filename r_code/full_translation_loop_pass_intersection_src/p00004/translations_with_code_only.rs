use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse as whitespace-separated tokens like scanf does.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        // scanf("%lf ...") reads 6 doubles; if it can't read them, it stops.
        let a = match it.next() {
            Some(s) => s.parse::<f64>().unwrap(),
            None => break,
        };
        let b = match it.next() {
            Some(s) => s.parse::<f64>().unwrap(),
            None => break,
        };
        let c = match it.next() {
            Some(s) => s.parse::<f64>().unwrap(),
            None => break,
        };
        let d = match it.next() {
            Some(s) => s.parse::<f64>().unwrap(),
            None => break,
        };
        let e = match it.next() {
            Some(s) => s.parse::<f64>().unwrap(),
            None => break,
        };
        let f = match it.next() {
            Some(s) => s.parse::<f64>().unwrap(),
            None => break,
        };

        // Preserve IEEE-754 behavior (division by zero -> inf, etc.) as in C double.
        let bai = d / a;
        let aa = a * bai;
        let bb = b * bai;
        let cc = c * bai;
        let y = (f - cc) / (e - bb);
        let x = (c - b * y) / a;

        // Match printf("%.3f %.3f\n", x, y)
        out.push_str(&format!("{:.3} {:.3}\n", x, y));

        // Keep variables to mirror C structure (aa is computed but unused in C too).
        let _ = aa;
    }

    print!("{out}");
}
