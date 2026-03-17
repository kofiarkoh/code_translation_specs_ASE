use std::io::{self, Write};

const TABAICHO_SIZE: usize = 8667;
const TABAICHO_BLOCK: i64 = 1_000_000_000;

struct Tabaicho {
    main: [i64; TABAICHO_SIZE],
}

impl Tabaicho {
    fn new() -> Self {
        Tabaicho {
            main: [0; TABAICHO_SIZE],
        }
    }

    fn set(&mut self, num: i64) {
        self.main[0] = num;
        for i in 1..TABAICHO_SIZE {
            self.main[i] = 0;
        }
    }

    fn mul(&mut self, num: i64) {
        let mut carry = 0;
        for i in 0..TABAICHO_SIZE {
            let temp = self.main[i] * num + carry;
            self.main[i] = temp % TABAICHO_BLOCK;
            carry = temp / TABAICHO_BLOCK;
        }
    }

    fn div(&mut self, num: i64) {
        let mut remainder = 0;
        for i in (0..TABAICHO_SIZE).rev() {
            remainder = remainder * TABAICHO_BLOCK + self.main[i];
            self.main[i] = remainder / num;
            remainder %= num;
        }
    }

    fn print(&self) {
        let mut started = false;
        for i in (0..TABAICHO_SIZE).rev() {
            if self.main[i] > 0 {
                print!("{}", self.main[i]);
                started = true;
                for j in (i - 1..TABAICHO_SIZE).rev() {
                    print!("{:09}", self.main[j]);
                }
                break;
            }
        }
        if !started {
            print!("0");
        }
        println!();
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut nums = input.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap());

    let n = nums.next().unwrap();
    let m = nums.next().unwrap();
    let mut r = nums.next().unwrap();
    
    r -= n * m; // Adjust r

    if r < 0 {
        println!("0");
    } else if r == 0 {
        println!("1");
    } else {
        let mut tabaicho = Tabaicho::new();
        tabaicho.set(1);
        let mut n2 = n - 1;

        if (n + r - 1) - (n - 1) < n - 1 {
            n2 = (n + r - 1) - (n - 1);
        }

        for i in 0..n2 {
            let j = n + r - 1 - i;
            tabaicho.mul(j);
        }

        for i in 1..=n2 {
            tabaicho.div(i);
        }

        tabaicho.print();
    }
}
