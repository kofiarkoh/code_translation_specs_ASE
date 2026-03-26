// AOJ 1310: Find the Multiples
// 2017.10.3 bal4u@uu

use std::io::{self, BufRead};

const HASHSIZ: usize = 300017;

#[derive(Default)]
struct Hash {
    b: i32,
    v: i32,
}

struct HashTable {
    table: Vec<Hash>,
}

impl HashTable {
    fn new() -> Self {
        HashTable {
            table: vec![Hash::default(); HASHSIZ + 2],
        }
    }

    fn lookup(&self, b: i32) -> i32 {
        let mut p = (b % HASHSIZ as i32) as usize;
        while self.table[p].b != 0 {
            if self.table[p].b == b {
                return self.table[p].v;
            }
            p = (p + 1) % HASHSIZ;
        }
        0
    }

    fn insert(&mut self, b: i32) {
        let mut p = (b % HASHSIZ as i32) as usize;
        while self.table[p].b != 0 {
            if self.table[p].b == b {
                self.table[p].v += 1;
                return;
            }
            p = (p + 1) % HASHSIZ;
        }
        self.table[p].b = b;
        self.table[p].v = 1;
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let mut tr = [[0; 6]; 10];
    tr[0][2] = 1;
    tr[2][2] = 1;
    tr[4][2] = 1;
    tr[6][2] = 1;
    tr[8][2] = 1;
    tr[0][5] = 1;
    tr[5][5] = 1;

    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let n = nums.next().unwrap();
        let s = nums.next().unwrap();
        let w = nums.next().unwrap();
        let q = nums.next().unwrap();

        if n <= 0 {
            break;
        }

        let mut g = s;
        let mut a = vec![0; n as usize];
        for i in 0..n {
            a[i as usize] = (g / 7) % 10;
            if g & 1 != 0 {
                g = (g >> 1) ^ w;
            } else {
                g >>= 1;
            }
        }

        let mut ans = 0;
        if q == 2 || q == 5 {
            let mut k = 0;
            for i in 0..n {
                if a[i as usize] != 0 {
                    k += 1;
                }
                if tr[a[i as usize] as usize][q as usize] != 0 {
                    ans += k;
                }
            }
        } else {
            let mut hash_table = HashTable::new();
            let mut b = vec![0; (n + 1) as usize];
            let mut k = 1;
            for i in (0..n).rev() {
                b[i as usize] = (k * a[i as usize] + b[(i + 1) as usize]) % q;
                k = ((k << 3) + (k << 1)) % q;
            }
            for i in 0..n {
                if a[i as usize] != 0 {
                    hash_table.insert(b[i as usize] + 1);
                }
                ans += hash_table.lookup(b[(i + 1) as usize] + 1);
            }
        }
        println!("{}", ans);
    }
}
