// AOJ 2061: International Party
// 2017.11.8 bal4u@uu

use std::io::{self, BufRead};
use std::collections::HashMap;

const HASHSIZ: usize = 61;

#[derive(Debug)]
struct HashEntry {
    s: Option<String>,
    id: i32,
}

struct HashTable {
    table: Vec<HashEntry>,
}

impl HashTable {
    fn new() -> Self {
        let mut table = Vec::with_capacity(HASHSIZ + 2);
        for _ in 0..HASHSIZ + 2 {
            table.push(HashEntry { s: None, id: 0 });
        }
        HashTable { table }
    }

    fn lookup(&self, s: &str) -> i32 {
        let w = s.len() as i32;
        let i = (101 * s.chars().next().unwrap() as i32 + 103 * s.chars().last().unwrap() as i32 + 107 * w) % HASHSIZ as i32;
        let mut tp = i;

        loop {
            if let Some(ref entry) = self.table[tp as usize].s {
                if entry == s {
                    return self.table[tp as usize].id;
                }
            } else {
                return -1;
            }
            tp = (tp + 1) % (HASHSIZ as i32 + 2);
        }
    }

    fn insert(&mut self, s: String, id: i32) -> i32 {
        let w = s.len() as i32;
        let i = (101 * s.chars().next().unwrap() as i32 + 103 * s.chars().last().unwrap() as i32 + 107 * w) % HASHSIZ as i32;
        let mut tp = i;

        loop {
            if let Some(ref entry) = self.table[tp as usize].s {
                if entry == &s {
                    return 0;
                }
            } else {
                self.table[tp as usize] = HashEntry { s: Some(s), id };
                return 1;
            }
            tp = (tp + 1) % (HASHSIZ as i32 + 2);
        }
    }
}

fn check(f: &[i32], m: usize, pat: i32) -> bool {
    let mut g = vec![0; 6];
    let mut s = 0;

    for i in 0..m {
        if f[i] & pat == 0 {
            return false;
        }
    }

    let mut k = pat;
    for i in 0..32 {
        if k & 1 != 0 {
            g[s] = 1 << i;
            s += 1;
        }
        k >>= 1;
    }

    for i in 0..m {
        for j in 0..s {
            g[j] |= f[i] & pat;
        }
    }

    let mut k = g[0];
    for i in 1..s {
        if g[i] & k != 0 {
            k |= g[i];
        }
    }
    k == pat
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let mut cno = 0;
    let mut hash_table = HashTable::new();
    let mut f = vec![0; 22];
    let mut m = 0;

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        let n = nums.next().unwrap();
        m = nums.next().unwrap();

        if n <= 0 {
            break;
        }

        if cno > 0 {
            println!();
        }
        hash_table = HashTable::new();
        f.fill(0);

        let mut sz = 0;
        let mut id = 1;

        for _ in 0..n {
            let name = lines.next().unwrap().unwrap();
            if hash_table.insert(name.clone(), id) == 1 {
                sz += 1;
                id <<= 1;
            }
        }

        for i in 0..m {
            let j: i32 = lines.next().unwrap().unwrap().parse().unwrap();
            for _ in 0..j {
                let buf = lines.next().unwrap().unwrap();
                id = hash_table.lookup(&buf);
                f[i] |= id;
            }
        }

        let mut ans = -1;
        for i in 1..=5 {
            let mut j = (1 << i) - 1;
            while j < (1 << sz) {
                if check(&f, m, j) {
                    ans = j;
                    id = i;
                    break;
                }
                let x = j & !(j - 1);
                let y = j + x;
                j = (((j & !y) / x) >> 1) | y;
            }
            if ans >= 0 {
                break;
            }
        }

        if ans < 0 {
            println!("Impossible");
        } else {
            println!("{}", id);
            for (i, name) in hash_table.table.iter().enumerate() {
                if ans & (1 << i) != 0 {
                    if let Some(ref s) = name.s {
                        println!("{}", s);
                    }
                }
            }
        }
        cno += 1;
    }
}
