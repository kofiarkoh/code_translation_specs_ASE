use std::io::{self, Read};

const MAX: usize = 50001;

fn make_prime_table(table: &mut [i32; MAX]) -> i32 {
    // table[i] == 0 means "prime" (or unmarked), table[i] == 1 means "composite"
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
    // Read entire stdin and parse integers similarly to repeated scanf("%d", &n).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut ptable = [0i32; MAX];
    make_prime_table(&mut ptable);

    let mut out = String::new();

    for tok in input.split_whitespace() {
        // If parsing fails, stop (scanf would stop matching).
        let n_i32: i32 = match tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        if n_i32 == 0 {
            break;
        }

        // The original C code assumes n is within range such that indices are valid.
        // We preserve behavior for valid inputs; for invalid (e.g., negative or too large),
        // C would have undefined behavior. Here we avoid panics by producing 0 pairs.
        if n_i32 < 0 || (n_i32 as usize) >= MAX {
            out.push_str("0\n");
            continue;
        }

        let n = n_i32 as usize;
        let mut cnt: i32 = 0;

        for i in 2..=(n / 2) {
            if ptable[i] == 0 && ptable[n - i] == 0 {
                cnt += 1;
            }
        }

        out.push_str(&format!("{cnt}\n"));
    }

    print!("{out}");
}
