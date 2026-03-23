use std::io::{self, Read};

static mut H: i32 = 0;
static mut W: i32 = 0;
static mut S: [[u8; 512]; 512] = [[0u8; 512]; 512];
static mut GROUP: [[i32; 512]; 512] = [[0i32; 512]; 512];
static mut COUNT: [[i32; 2]; 512 * 512] = [[0i32; 2]; 512 * 512];

fn dfs(start_y: i32, start_x: i32, mikisinn: i32) {
    // The original C uses recursion; to avoid stack overflow in Rust, use an explicit stack.
    // Semantics are preserved: same reachability and group assignment.
    const D: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut stack: Vec<(i32, i32)> = Vec::new();
    stack.push((start_y, start_x));

    unsafe {
        while let Some((y, x)) = stack.pop() {
            if GROUP[y as usize][x as usize] == mikisinn {
                continue;
            }
            GROUP[y as usize][x as usize] = mikisinn;

            let cur = S[y as usize][x as usize];
            for (dy, dx) in D {
                let ny = y + dy;
                let nx = x + dx;
                if 0 <= ny && ny < H && 0 <= nx && nx < W {
                    if cur != S[ny as usize][nx as usize] {
                        if GROUP[ny as usize][nx as usize] != mikisinn {
                            stack.push((ny, nx));
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    // Read entire stdin, tokenize like scanf("%d%d") and scanf("%s").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => std::process::exit(1),
    };
    let w: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => std::process::exit(1),
    };

    unsafe {
        H = h;
        W = w;
    }

    for i in 0..(h as usize) {
        let row = match it.next() {
            Some(v) => v.as_bytes(),
            None => std::process::exit(1),
        };
        // C uses "%511s" into S[i]; we store up to W bytes (W <= 512 implied by arrays).
        for j in 0..(w as usize) {
            unsafe {
                S[i][j] = row[j];
            }
        }
    }

    let mut asumisu: i32 = 0;
    for i in 0..(h as usize) {
        for j in 0..(w as usize) {
            let g = unsafe { GROUP[i][j] };
            if g == 0 {
                asumisu += 1;
                dfs(i as i32, j as i32, asumisu);
            }
        }
    }

    for i in 0..(h as usize) {
        for j in 0..(w as usize) {
            unsafe {
                let gid = GROUP[i][j] as usize;
                let is_dot = (S[i][j] == b'.') as usize; // matches C: S[i][j] == '.'
                COUNT[gid][is_dot] += 1;
            }
        }
    }

    let mut mizuhasu: u64 = 0;
    for i in 1..=(asumisu as usize) {
        unsafe {
            mizuhasu = mizuhasu.wrapping_add(
                (COUNT[i][0] as u64).wrapping_mul(COUNT[i][1] as u64),
            );
        }
    }

    print!("{}\n", mizuhasu);
}
