use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut buffer = String::new();

    while input.read_line(&mut buffer).unwrap() > 0 {
        let mut parts = buffer.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let m: usize = parts.next().unwrap().parse().unwrap();

        if n == 0 && m == 0 {
            break;
        }

        let mut e = vec![String::new(); n];
        let mut len = vec![0; n];
        let mut t = String::new();
        let mut flag = vec![vec![false; 1 << n]; 5001];

        for i in 0..n {
            buffer.clear();
            input.read_line(&mut buffer).unwrap();
            e[i] = buffer.trim().to_string();
            len[i] = e[i].len();
        }

        for _ in 0..m {
            buffer.clear();
            input.read_line(&mut buffer).unwrap();
            let mut buf = buffer.trim().to_string();
            if let Some(last_char) = buf.chars().last() {
                if last_char < ' ' {
                    buf.pop(); // Remove the last character if it's a whitespace
                }
            }
            t.push_str(&buf);
        }

        let l_s = t.len();
        let loop_count = 1 << n;
        let mut cnt = 0;

        for i in 0..l_s {
            for j in 0..loop_count {
                if j != 0 && !flag[i][j] {
                    continue;
                }
                for k in 0..n {
                    let mask = 1 << k;
                    if j & mask != 0 {
                        continue;
                    }
                    if &t[i..].starts_with(&e[k]) {
                        let pat = j | mask;
                        flag[i + len[k]][pat] = true;
                    }
                }
            }
        }

        for i in 0..=l_s {
            cnt += flag[i][loop_count - 1] as usize;
        }

        println!("{}", cnt);
        buffer.clear(); // Clear buffer for the next iteration
    }
}
```

### Notes:
1. **Input Handling**: The Rust code uses `BufRead` for efficient reading of input, similar to how `scanf` is used in C.
2. **String Handling**: The `String` type in Rust is used to manage dynamic strings, and `push_str` is used to concatenate strings.
3. **Flag Structure**: The `flag` array is implemented as a vector of vectors, maintaining the same structure as the C code.
4. **Loop Logic**: The logic for checking prefixes and updating the `flag` structure is preserved, ensuring that the semantics of the original C code are maintained.
5. **Whitespace Handling**: The code checks for trailing whitespace and removes it, similar to the C code's behavior.
6. **Safety**: The Rust code avoids any `unsafe` blocks, adhering to the requirement for safe and idiomatic Rust.
