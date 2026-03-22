use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse as whitespace-separated tokens (like scanf).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(tok) => tok.parse().unwrap(),
        None => return,
    };

    // In the C code, S and T are read with "%s" into char arrays of size N+1.
    // We emulate by reading tokens; if missing, treat as empty.
    let s = it.next().unwrap_or("").as_bytes().to_vec();
    let t = it.next().unwrap_or("").as_bytes().to_vec();

    let mut ans: i32 = 0;

    // The C code has a logic bug: it declares `int i=N-1; int j=i;`
    // then enters `while (i>=0)` but *redeclares* `int i=N-1;` inside,
    // so the outer `i` never changes and the loop would be infinite unless it breaks.
    // We preserve that behavior by modeling the outer loop as an infinite loop that
    // only exits via the same break conditions.
    let mut j: i32 = n - 1;

    loop {
        // Inner `i` shadows the outer `i` in C; it is reset each iteration.
        let mut i: i32 = n - 1;

        while j >= 0 {
            let si = s.get(i as usize).copied().unwrap_or(0);
            let tj = t.get(j as usize).copied().unwrap_or(0);
            if si == tj {
                break;
            }
            j -= 1;
        }

        if j >= 0 {
            while j >= 0 {
                let si = s.get(i as usize).copied().unwrap_or(0);
                let tj = t.get(j as usize).copied().unwrap_or(0);
                if si != tj {
                    break;
                }
                i -= 1;
                j -= 1;
            }
            if j < 0 {
                // ans = 2*N - (N - i - 1)
                // Use wrapping to mimic C signed overflow behavior as closely as possible.
                ans = (2i32)
                    .wrapping_mul(n)
                    .wrapping_sub(n.wrapping_sub(i).wrapping_sub(1));
                break;
            }
        } else {
            ans = (2i32)
                .wrapping_mul(n)
                .wrapping_sub(n.wrapping_sub(i).wrapping_sub(1));
            break;
        }
    }

    // C prints without a trailing newline.
    print!("{}", ans);
}
