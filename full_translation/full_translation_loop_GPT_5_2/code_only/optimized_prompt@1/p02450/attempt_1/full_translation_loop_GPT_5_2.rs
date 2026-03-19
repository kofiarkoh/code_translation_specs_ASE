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
        print!("\n");
        return;
    }

    // In the C code: int used2[n-d-1];
    // This is a VLA sized to (n - d - 1). When n==d, this would be negative,
    // but that path is avoided by the early return above.
    // Here we allocate a Vec of the same logical length.
    let mut used2: Vec<i32> = vec![0; n - d - 1];

    for i in 0..(n - d) {
        result[d] = used[i];
        let mut k = 0usize;
        for j in 0..(n - d) {
            if j != i {
                used2[k] = used[j];
                k += 1;
            }
        }
        enumeration(result, &used2, n, d + 1);
    }
}

fn main() {
    // Read entire stdin; C code uses scanf("%d\n",&n) which reads an int and
    // then consumes trailing whitespace. Splitting by whitespace matches that.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse::<i32>().unwrap() as usize,
        None => return, // If no input, do nothing (closest to failing scanf).
    };

    let mut used: Vec<i32> = (1..=(n as i32)).collect();
    let mut result: Vec<i32> = vec![0; n];

    enumeration(&mut result, &used, n, 0);
}
