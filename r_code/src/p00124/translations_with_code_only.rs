use std::io::{self, Write};

fn main() {
    let mut a = [0; 11];
    let mut b = [0; 11];
    let mut c = [0; 11];
    let mut ten = [0; 11];
    let mut moziretsu = [[0u8; 11]; 21];
    let mut mtemp = [0u8; 11];
    let mut cnt = 0;

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut n: i32 = input.trim().parse().unwrap();

    while n != 0 {
        if cnt > 0 {
            println!(); // Print a newline if this is not the first output
        }

        for i in 0..n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let parts: Vec<&str> = input.trim().split_whitespace().collect();
            moziretsu[i].copy_from_slice(parts[0].as_bytes());
            a[i] = parts[1].parse().unwrap();
            b[i] = parts[2].parse().unwrap();
            c[i] = parts[3].parse().unwrap();
            ten[i] = a[i] * 3 + b[i] * 0 + c[i] * 1; // Equivalent to a[i] * 3 + c[i]
        }

        // Sort ten and corresponding moziretsu
        for i in 0..n - 1 {
            for j in (i + 1)..n {
                if ten[i] < ten[j] {
                    // Swap ten
                    let temp = ten[i];
                    ten[i] = ten[j];
                    ten[j] = temp;

                    // Swap corresponding moziretsu
                    let temp_str = moziretsu[i];
                    moziretsu[i] = moziretsu[j];
                    moziretsu[j] = temp_str;
                }
            }
        }

        for i in 0..n {
            // Convert bytes back to string for output
            let name = String::from_utf8_lossy(&moziretsu[i]);
            println!("{},{}", name, ten[i]);
        }

        cnt += 1;

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        n = input.trim().parse().unwrap();
    }
}
