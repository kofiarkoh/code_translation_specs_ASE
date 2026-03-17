use std::io::{self, BufRead};

const MAX: usize = 50001;

fn make_prime_table(table: &mut [i32; MAX]) -> i32 {
    for i in 0..MAX {
        table[i] = 0;
    }

    for i in 2..MAX {
        if table[i] == 0 {
            for j in 2..(MAX / i) {
                table[i * j] = 1;
            }
        }
    }
    0
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut n = String::new();
    let mut ptable = [0; MAX];

    make_prime_table(&mut ptable);

    while input.read_line(&mut n).unwrap() > 0 {
        let n: i32 = n.trim().parse().unwrap();
        if n == 0 {
            break;
        } else {
            let mut cnt = 0;
            for i in 2..=n / 2 {
                if ptable[i as usize] == 0 && ptable[(n - i) as usize] == 0 {
                    cnt += 1;
                }
            }
            println!("{}", cnt);
        }
        n.clear(); // Clear the buffer for the next input
    }
}
