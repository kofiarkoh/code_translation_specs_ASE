use std::io::{self, Read};

const MAX: usize = 100001;

#[derive(Copy, Clone, Default)]
struct Rect {
    height: i64,
    pos: i64,
}

static mut TABLE: [i64; MAX] = [0; MAX];

fn large_histogram(w: i64) -> i64 {
    let mut sp: usize = 0;
    let mut max_area: i64 = 0;
    let mut stack: [Rect; MAX] = [Rect::default(); MAX];

    // Matches C: table[w] = 0; (sentinel)
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
                // Use wrapping ops to emulate C signed overflow behavior as closely as possible.
                let width = i.wrapping_sub(pre.pos);
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
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // The original C code assumes w <= MAX-1; we preserve that assumption.
    for i in 0..w {
        let v: i64 = it.next().unwrap().parse().unwrap();
        unsafe {
            TABLE[i as usize] = v;
        }
    }

    let temp = large_histogram(w);
    let mut max_val: i64 = 0;
    if max_val < temp {
        max_val = temp;
    }
    print!("{max_val}\n");
}
