use std::io::{self, Read};

fn main() {
    // Read entire stdin, then mimic scanf("%s", str) by taking the first
    // whitespace-delimited token (or empty if none).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let token = input.split_whitespace().next().unwrap_or("");

    // C: char str[1010]; int a[30] = {};
    // We'll keep a fixed-size array of 30 ints (0/1), toggled via ! in C.
    let mut a = [0i32; 30];

    // C toggles: a[str[i]-'a'] = !a[str[i]-'a'];
    // In C, !0 -> 1, !nonzero -> 0. Here values stay 0/1, so 1 - a[idx] works.
    // Note: The C code has undefined behavior if characters are outside 'a'..'z'
    // (negative/out-of-bounds index). We assume inputs follow that implied constraint.
    for &b in token.as_bytes() {
        let idx = (b as i32 - b'a' as i32) as usize;
        a[idx] = 1 - a[idx];
    }

    let sum: i32 = a.iter().sum();
    println!("{}", sum / 2);
}
