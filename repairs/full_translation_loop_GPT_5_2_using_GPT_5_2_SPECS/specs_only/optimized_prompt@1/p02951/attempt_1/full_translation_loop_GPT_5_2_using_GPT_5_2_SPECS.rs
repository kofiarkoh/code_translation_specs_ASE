use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i128 = it.next().unwrap().parse().unwrap();
    let b: i128 = it.next().unwrap().parse().unwrap();
    let c: i128 = it.next().unwrap().parse().unwrap();

    let d = a - b;

    let c_prime = if d <= 0 {
        c
    } else if c <= 0 {
        // When C <= 0 and D > 0, the specification states C' = C - D (no early stop at zero).
        c - d
    } else {
        // When C > 0 and D > 0, decrease by up to D steps, stopping early if reaching zero.
        let dec = if d < c { d } else { c };
        c - dec
    };

    print!("{c_prime}");
}
