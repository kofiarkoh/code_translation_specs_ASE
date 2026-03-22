use std::io::{self, Read};

const MAX: usize = 50001;

fn make_prime_table(table: &mut [i32; MAX]) -> i32 {
    for i in 0..MAX {
        table[i] = 0;
    }

    for i in 2..MAX {
        if table[i] == 0 {
            let mut j: usize = 2;
            while i * j < MAX {
                table[i * j] = 1;
                j += 1;
            }
        }
    }
    0
}

fn main() {
    // Read entire stdin, then parse ints similarly to repeated scanf("%d", &n).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut ptable = [0i32; MAX];
    make_prime_table(&mut ptable);

    for tok in input.split_whitespace() {
        // C's scanf("%d") reads into int; emulate by parsing i32.
        // If parsing fails, stop (closest to scanf returning != 1).
        let n: i32 = match tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        if n == 0 {
            break;
        }

        let mut cnt: i32 = 0;

        // C loop: for (i = 2; i <= n/2; i++)
        // Note: if n < 0, n/2 is <= 0, so loop doesn't run (same as C).
        let half = n / 2;
        let mut i: i32 = 2;
        while i <= half {
            // In the original C, indexing with negative or out-of-range values would be UB.
            // We preserve behavior for valid inputs (0 <= i, n-i < MAX).
            let ni = n - i;
            if i >= 0 && ni >= 0 {
                let iu = i as usize;
                let niu = ni as usize;
                if iu < MAX && niu < MAX && ptable[iu] == 0 && ptable[niu] == 0 {
                    cnt += 1;
                }
            }
            i += 1;
        }

        print!("{cnt}\n");
    }
}
