use std::io::{self, Read};

const BUF_SIZE: usize = 20;

fn main() {
    // Emulate the C program's line-based input with a fixed buffer size (BUF_SIZE).
    // In C: fgets(line, BUF_SIZE, stdin) reads at most BUF_SIZE-1 chars.
    // Then sscanf("%d %d", ...) parses from that line only.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Take only the first line, and only up to BUF_SIZE-1 bytes, matching fgets behavior.
    let first_line = input.split_inclusive('\n').next().unwrap_or("");
    let mut line = first_line.as_bytes();
    if line.len() > BUF_SIZE.saturating_sub(1) {
        line = &line[..BUF_SIZE - 1];
    }
    let line_str = std::str::from_utf8(line).unwrap_or("");

    // Mimic sscanf("%d %d", &n, &k) with default-initialized ints if parsing fails.
    // In the C code, n and k are uninitialized if sscanf fails; that's UB.
    // We choose a deterministic, safe behavior: treat missing/invalid as 0.
    let mut it = line_str.split_whitespace();
    let n: i32 = it.next().and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
    let k: i32 = it.next().and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);

    // Main logic: iterate pat from 0 to (1<<n)-1, filter by popcount == k.
    // In C, (1<<n) is undefined for n<0 or n>=bitwidth; we avoid panics and
    // produce an empty range in those cases.
    let mut out = String::new();

    if n >= 0 && n < 31 {
        let limit: i32 = 1i32.wrapping_shl(n as u32); // safe for n in [0,30]
        for pat in 0..limit {
            if (pat as u32).count_ones() as i32 != k {
                continue;
            }
            out.push_str(&format!("{}:", pat));
            for i in 0..n {
                // (1<<i) is safe here since i in [0, n-1] and n<31
                if (pat & (1i32 << (i as u32))) == 0 {
                    continue;
                }
                out.push_str(&format!(" {}", i));
            }
            out.push('\n');
        }
    }

    print!("{out}");
}
