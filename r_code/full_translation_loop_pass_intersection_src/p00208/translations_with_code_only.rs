use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse integers similarly to repeated scanf("%d", &a).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut n = [0i32; 200];

    loop {
        let a_opt = it.next();
        if a_opt.is_none() {
            break; // EOF: C code would stop making progress; we terminate.
        }
        let mut a: i32 = a_opt.unwrap().parse().unwrap();

        if a == 0 {
            break;
        }

        let mut b: i32 = 0;
        // Matches C behavior for positive a. For negative a, C's / and % truncate toward zero;
        // Rust does the same for i32.
        while a != 0 {
            n[b as usize] = a % 8;
            a /= 8;
            b += 1;
        }

        n[b as usize] = 0;

        let mut i: i32 = 0;
        while i <= b {
            let idx = i as usize;
            if n[idx] > 3 {
                n[idx] += 1;
            }
            if n[idx] > 5 {
                n[idx] += 1;
            }
            i += 1;
        }

        // Print digits from most significant to least, exactly as C loop does.
        // Note: In C, if b==0 then i starts at -1 and the loop doesn't run; same here.
        let mut out = String::new();
        let mut i = b - 1;
        while i >= 0 {
            out.push_str(&format!("{}", n[i as usize]));
            i -= 1;
        }
        out.push('\n');
        print!("{out}");
    }
}
