use std::io::{self, Read};

const MAX: usize = 100001;

#[derive(Copy, Clone, Debug)]
struct Rect {
    height: i64,
    pos: i64,
}

static mut TABLE: [i64; MAX] = [0; MAX];

fn large_histogram(w: i64) -> i64 {
    let mut sp: usize = 0;
    let mut max_area: i64 = 0;
    let mut stack: [Rect; MAX] = [Rect { height: 0, pos: 0 }; MAX];

    // The C code writes table[w] = 0 as a sentinel.
    // This assumes 0 <= w < MAX; we preserve that behavior.
    unsafe {
        TABLE[w as usize] = 0;
    }

    for i in 0..=w {
        let mut r = Rect {
            height: unsafe { TABLE[i as usize] },
            pos: i,
        };

        if sp == 0 || stack[sp - 1].height < r.height {
            stack[sp] = r;
            sp += 1;
        } else if stack[sp - 1].height > r.height {
            let mut target = i;
            while sp != 0 && stack[sp - 1].height >= r.height {
                sp -= 1;
                let pre = stack[sp];
                // Use wrapping arithmetic to emulate C signed overflow behavior as closely as possible.
                let width = (i - pre.pos).wrapping_mul(1);
                let area = pre.height.wrapping_mul(width);
                if max_area < area {
                    max_area = area;
                }
                target = pre.pos;
            }
            r.pos = target;
            stack[sp] = r;
            sp += 1;
        }
    }

    max_area
}

fn main() {
    // Read all input, parse as i64 like scanf("%lld").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let w: i64 = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return, // If no input, C would have undefined behavior; we just exit.
    };

    // Fill TABLE[0..w)
    for i in 0..w {
        if let Some(x) = it.next() {
            let v: i64 = x.parse().unwrap();
            unsafe {
                TABLE[i as usize] = v;
            }
        } else {
            // Missing inputs: C would leave uninitialized; we keep default 0.
            break;
        }
    }

    let temp = large_histogram(w);
    let mut maxv: i64 = 0;
    if maxv < temp {
        maxv = temp;
    }
    print!("{maxv}\n");
}
