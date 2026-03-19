use std::io::{self, Read};

fn enumeration(result: &mut [i32], used: &[i32], n: usize, d: usize) {
    if d >= n {
        for i in 0..n {
            if i == 0 {
                print!("{}", result[i]);
            } else {
                print!(" {}", result[i]);
            }
        }
        println!();
        return;
    }

    // In the C code: int used2[n-d-1];
    // When d == n-1, this is a VLA of size 0, which is allowed in many C compilers.
    // In Rust, we model it with a Vec of length (n - d - 1), which becomes 0 in that case.
    let len_used2 = n - d - 1;

    for i in 0..(n - d) {
        result[d] = used[i];

        let mut used2: Vec<i32> = Vec::with_capacity(len_used2);
        for j in 0..(n - d) {
            if j != i {
                used2.push(used[j]);
            }
        }

        enumeration(result, &used2, n, d + 1);
    }
}

fn main() {
    // Read entire stdin; C code uses scanf("%d\n", &n) which reads an int and then
    // consumes trailing whitespace. Splitting by whitespace matches that behavior.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_i32: i32 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // The C code uses VLAs sized by n. Negative n would be undefined/invalid there.
    // We emulate typical behavior by treating negative as 0 to avoid panics.
    let n: usize = if n_i32 > 0 { n_i32 as usize } else { 0 };

    let mut used: Vec<i32> = (1..=n as i32).collect();
    let mut result: Vec<i32> = vec![0; n];

    enumeration(&mut result, &used, n, 0);
}
