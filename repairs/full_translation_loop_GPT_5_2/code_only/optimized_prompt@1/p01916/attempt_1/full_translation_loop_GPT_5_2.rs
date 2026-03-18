use std::io::{self, Read};

fn main() {
    // Read entire stdin, then mimic scanf("%s", str) by taking the first
    // whitespace-delimited token (or empty if none).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let token = input.split_whitespace().next().unwrap_or("");

    // C: char str[1010]; int a[30] = {};
    // We'll keep a fixed-size array of 30 ints (0/1 toggles).
    let mut a = [0i32; 30];

    // C loop: for(i=0; str[i]; i++) a[str[i]-'a'] = !a[str[i]-'a'];
    // Note: In C, indexing is based on bytes; we emulate that by iterating bytes.
    // If input contains bytes outside 'a'..'z', the C code would index out of bounds
    // (undefined behavior). We do not add checks to preserve the "assume valid" behavior.
    for &b in token.as_bytes() {
        let idx = (b.wrapping_sub(b'a')) as usize;
        a[idx] = if a[idx] == 0 { 1 } else { 0 };
    }

    // C: sum over 30 entries, then print sum/2.
    let sum: i32 = a.iter().sum();
    println!("{}", sum / 2);
}
