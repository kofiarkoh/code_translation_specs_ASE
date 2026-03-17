// AOJ 1370 Hidden Anagrams
// Translated from C to Rust

use std::collections::HashMap;
use std::io::{self, BufRead};

const HASHSIZ: usize = 10007;

#[derive(Default)]
struct Hash {
    f: [i16; 26],
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

    fn insert(&mut self, f: &mut [i16; 26]) {
        f[0] += 1;
        let mut k: u64 = 0;
        for i in (0..26).step_by(2) {
            k = (k << 4) + f[i as usize] as u64;
        }
        let mut tp = &mut self.table[(k % HASHSIZ as u64) as usize];

        while tp.f[0] != 0 {
            if tp.f == *f {
                return; // Already exists
            }
            tp = &mut self.table[(tp as *mut _ as usize + 1 - self.table.as_ptr() as usize) % self.table.len()];
        }
        tp.f.copy_from_slice(f);
        f[0] -= 1;
    }

    fn lookup(&mut self, f: &mut [i16; 26]) -> bool {
        f[0] += 1;
        let mut k: u64 = 0;
        for i in (0..26).step_by(2) {
            k = (k << 4) + f[i as usize] as u64;
        }
        let mut tp = &mut self.table[(k % HASHSIZ as u64) as usize];

        while tp.f[0] != 0 {
            if tp.f == *f {
                f[0] -= 1;
                return true; // Found
            }
            tp = &mut self.table[(tp as *mut _ as usize + 1 - self.table.as_ptr() as usize) % self.table.len()];
        }
        f[0] -= 1;
        false
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut sa = String::new();
    let mut sb = String::new();

    reader.read_line(&mut sa).unwrap();
    reader.read_line(&mut sb).unwrap();

    let wa = sa.trim().len();
    let wb = sb.trim().len();
    let mut w = wa.min(wb);

    let mut hash_table = HashTable::new();
    let mut f = [0; 26];

    for k in (1..=w).rev() {
        // Reset hash table
        hash_table = HashTable::new();
        f = [0; 26];

        // Process first string
        for c in sa.chars().take(k) {
            f[(c as usize - 'a' as usize)] += 1;
        }
        hash_table.insert(&mut f);

        let mut i = wa - k;
        let mut p = sa.chars();
        for _ in 0..i {
            f[(p.next().unwrap() as usize - 'a' as usize)] -= 1;
            f[(p.next().unwrap() as usize - 'a' as usize)] += 1;
            hash_table.insert(&mut f);
        }

        f = [0; 26];
        for c in sb.chars().take(k) {
            f[(c as usize - 'a' as usize)] += 1;
        }
        if hash_table.lookup(&mut f) {
            println!("{}", k);
            return;
        }

        i = wb - k;
        let mut q = sb.chars();
        for _ in 0..i {
            f[(q.next().unwrap() as usize - 'a' as usize)] -= 1;
            f[(q.next().unwrap() as usize - 'a' as usize)] += 1;
            if hash_table.lookup(&mut f) {
                println!("{}", k);
                return;
            }
        }
    }
}
