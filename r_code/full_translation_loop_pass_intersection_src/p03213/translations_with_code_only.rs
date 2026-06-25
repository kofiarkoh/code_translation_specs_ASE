use std::io::{self, Read};

static mut N: i32 = 0;
static mut YAKU: [i32; 101] = [0; 101];

fn num(a: i32) -> i32 {
    // Uses global N and YAKU exactly like the C code.
    let mut c: i32 = 0;
    unsafe {
        for i in 2..=N {
            let idx = i as usize;
            if YAKU[idx] >= a - 1 {
                c += 1;
            }
        }
    }
    c
}

fn main() {
    // Read entire stdin and parse first integer, matching scanf("%d", &n) behavior
    // for typical valid inputs (the C code does not handle invalid input).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i32 = it.next().unwrap().parse().unwrap();

    // We use `unsafe` only to mirror the C globals (N, YAKU) without changing structure.
    unsafe {
        N = n;

        for i in 2..=N {
            let mut a = i;
            let mut waru: i32 = 2;
            while a > 1 {
                while a % waru == 0 {
                    a /= waru;
                    let w = waru as usize;
                    if w < YAKU.len() {
                        YAKU[w] += 1;
                    }
                }
                waru += 1;
            }
        }
    }

    // Keep i32 arithmetic; for the given array bounds and typical constraints this won't overflow.
    // (C would have undefined behavior on signed overflow; we do not introduce wrapping.)
    let ans: i32 = num(75)
        + num(25) * (num(3) - 1)
        + num(15) * (num(5) - 1)
        + num(5) * (num(5) - 1) * (num(3) - 2) / 2;

    print!("{ans}\n");
}
