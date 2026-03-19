use std::io::{self, Read};

const NUM_MAX: usize = 1000;
const K_MAX: usize = 1000;
const DIVISOR: u64 = 1_000_000_007;

fn init() -> [[u64; NUM_MAX + 1]; K_MAX + 1] {
    let mut pdp = [[0u64; NUM_MAX + 1]; K_MAX + 1];
    pdp[0][0] = 1;

    for i in 1..=K_MAX {
        for j in 0..=NUM_MAX {
            let mut v = pdp[i - 1][j];
            if j >= i {
                v = (v + pdp[i][j - i]) % DIVISOR;
            }
            pdp[i][j] = v;
        }
    }
    pdp
}

fn main() {
    // Emulate the C behavior: read a line (up to BUF_SIZE=20) and parse two ints.
    // If input is missing, C would return -1 from get_int2 but main ignores it;
    // n/k would be uninitialized (UB). In Rust we must initialize; we default to 0,0
    // if parsing fails, which is the closest safe approximation.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    // C uses fgets(line, 20, stdin): it reads at most 19 chars plus NUL, stopping at newline.
    // Approximate by taking the first line and truncating to 19 bytes (ASCII digits/spaces expected).
    let first_line = input.lines().next().unwrap_or("");
    let truncated = if first_line.as_bytes().len() > 19 {
        // Safe because we only slice at UTF-8 boundary; input is expected ASCII.
        // If not, fall back to the whole line.
        first_line
            .char_indices()
            .take_while(|&(idx, _)| idx < 19)
            .map(|(_, ch)| ch)
            .collect::<String>()
    } else {
        first_line.to_string()
    };

    let mut it = truncated.split_whitespace();
    let n: usize = it.next().and_then(|s| s.parse::<i64>().ok()).unwrap_or(0).max(0) as usize;
    let k: usize = it.next().and_then(|s| s.parse::<i64>().ok()).unwrap_or(0).max(0) as usize;

    let pdp = init();

    // In C, indexing out of bounds would be UB. Here we clamp to avoid panic while keeping
    // behavior defined; for valid inputs (0..=1000) this matches exactly.
    let nn = n.min(NUM_MAX);
    let kk = k.min(K_MAX);

    // C prints (int)pdp[k][n]; values are < DIVISOR, so this fits in i32.
    println!("{}", pdp[kk][nn] as i32);
}
