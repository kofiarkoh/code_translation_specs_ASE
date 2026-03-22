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
    // This is a VLA sized for the next recursion level (where used length becomes n-(d+1)).
    // Here we allocate a Vec of that exact length.
    let used2_len = n - d - 1;

    // In the C code: for(i=0;i<n-d;i++) { ... }
    // Note: used is expected to have length n-d at each level.
    for i in 0..(n - d) {
        result[d] = used[i];

        let mut used2: Vec<i32> = Vec::with_capacity(used2_len);
        for j in 0..(n - d) {
            if j != i {
                used2.push(used[j]);
            }
        }
        // used2 should now have length used2_len.
        enumeration(result, &used2, n, d + 1);
    }
}

fn main() {
    // Read entire stdin; scanf("%d\n",&n) reads an int and then consumes trailing whitespace.
    // Using split_whitespace matches the effective behavior for this program.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_i32: i32 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // If no input, C would have undefined behavior; we just exit.
    };

    // The C code uses VLAs with size n; negative n is undefined in C.
    // We emulate the "intended" domain by requiring n >= 0.
    let n: usize = n_i32.try_into().unwrap();

    let mut used: Vec<i32> = (1..=n as i32).collect();
    let mut result: Vec<i32> = vec![0; n];

    enumeration(&mut result, &used, n, 0);
}
