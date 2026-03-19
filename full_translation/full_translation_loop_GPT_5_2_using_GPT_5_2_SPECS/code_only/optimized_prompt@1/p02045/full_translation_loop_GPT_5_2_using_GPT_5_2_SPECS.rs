use std::io::{self, Read};

static mut A: i32 = 0;
static mut B: i32 = 0;
static mut C: i32 = 0;
static mut D: i32 = 0;

fn nigeru(x: i32, y: i32, is_src: bool) -> i32 {
    // Mirrors the C logic exactly, using i32 arithmetic.
    // Note: C's signed overflow is UB; we assume inputs keep computations in range.
    let (a, b, c, d) = unsafe { (A, B, C, D) };

    let mut cost: i32 = 0;
    if x < a {
        cost += a - x;
    }
    if c < x {
        cost += x - c;
    }
    if y < b {
        cost += b - y;
    }
    if d < y {
        cost += y - d;
    }
    if is_src && cost > 0 {
        cost -= 1;
    }
    cost
}

fn abs_i32(v: i32) -> i32 {
    // C's abs(int) is undefined for INT_MIN; Rust's i32::abs would panic in debug.
    // We emulate typical two's-complement behavior by using wrapping_abs.
    v.wrapping_abs()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = match n_opt.unwrap().parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        if n <= 0 {
            break;
        }

        let a: i32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => return,
        };
        let b: i32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => return,
        };
        let c: i32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => return,
        };
        let d: i32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => return,
        };

        unsafe {
            A = a;
            B = b;
            C = c;
            D = d;
        }

        let n_usize = n as usize;
        // C uses fixed arrays of size 128; preserve that bound.
        let mut x = [0i32; 128];
        let mut y = [0i32; 128];

        for i in 0..=n_usize {
            let xi: i32 = match it.next().and_then(|s| s.parse().ok()) {
                Some(v) => v,
                None => return,
            };
            let yi: i32 = match it.next().and_then(|s| s.parse().ok()) {
                Some(v) => v,
                None => return,
            };
            // If N > 127, the C code would write out of bounds (UB).
            // We avoid panicking by only storing when in range; behavior for such inputs
            // is inherently undefined in C, so this is a best-effort containment.
            if i < 128 {
                x[i] = xi;
                y[i] = yi;
            }
        }

        let mut mikakosi: i32 = 0;
        for i in 1..=n_usize {
            if i >= 128 {
                // Past the fixed buffer; C behavior is UB. Stop to avoid panic.
                break;
            }
            let naka = abs_i32(x[i - 1].wrapping_sub(x[i]))
                .wrapping_add(abs_i32(y[i - 1].wrapping_sub(y[i])));
            let simakaze = nigeru(x[i - 1], y[i - 1], true).wrapping_add(nigeru(x[i], y[i], false));
            mikakosi = mikakosi.wrapping_add(if naka < simakaze { naka } else { simakaze });
        }

        out.push_str(&format!("{mikakosi}\n"));
    }

    print!("{out}");
}
