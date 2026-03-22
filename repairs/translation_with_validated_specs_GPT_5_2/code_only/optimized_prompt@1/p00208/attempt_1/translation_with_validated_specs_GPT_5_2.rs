use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse ints like repeated scanf("%d", &a).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    // Mirrors C locals; keep i32 to match typical C int behavior.
    let mut n = [0i32; 200];

    loop {
        let a_opt = it.next();
        if a_opt.is_none() {
            break; // EOF: C would stop making progress; we terminate.
        }
        let mut a: i32 = a_opt.unwrap().parse().unwrap();

        if a == 0 {
            break;
        }

        let mut b: i32 = 0;

        // Convert to base-8 digits, least significant first.
        // Note: For negative a, C's % and / truncate toward zero; Rust matches that.
        while a != 0 {
            let idx = b as usize;
            n[idx] = a % 8;
            a /= 8;
            b += 1;
        }

        // Sentinel digit as in C.
        n[b as usize] = 0;

        // c is unused in C; omit.
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

        // Print digits from most significant to least, excluding the sentinel at n[b].
        // C loop: for(i=b-1;i>=0;i--) with signed int; if b==0, it prints nothing.
        if b > 0 {
            for i in (0..(b as usize)).rev() {
                out.push_str(&format!("{}", n[i]));
            }
        }
        out.push('\n');
    }

    print!("{out}");
}
