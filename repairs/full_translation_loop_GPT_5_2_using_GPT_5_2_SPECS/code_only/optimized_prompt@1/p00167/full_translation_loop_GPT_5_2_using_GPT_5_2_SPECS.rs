use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut a = [0i32; 100];

    loop {
        let n_opt = it.next().and_then(|s| s.parse::<i32>().ok());
        let n = match n_opt {
            Some(v) => v,
            None => break, // scanf failed / EOF
        };
        if n == 0 {
            break; // matches `&& n` in the while condition
        }

        // The C code assumes n <= 100; we preserve behavior for valid inputs.
        let n_usize = n as usize;

        for i in 0..n_usize {
            if let Some(tok) = it.next() {
                a[i] = tok.parse::<i32>().unwrap();
            } else {
                // If input is malformed/short, C's behavior is undefined; we stop.
                return;
            }
        }

        let mut swap_count: i32 = 0;
        for u in (1..n_usize).rev() {
            for i in 0..u {
                if a[i] > a[i + 1] {
                    let t = a[i];
                    a[i] = a[i + 1];
                    a[i + 1] = t;
                    // In C, signed overflow is UB; assume counts fit in i32 for valid inputs.
                    swap_count += 1;
                }
            }
        }

        print!("{swap_count}\n");
    }
}
