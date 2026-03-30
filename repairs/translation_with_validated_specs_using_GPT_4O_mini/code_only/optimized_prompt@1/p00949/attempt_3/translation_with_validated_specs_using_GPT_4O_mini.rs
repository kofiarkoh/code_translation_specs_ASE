// AOJ 1370 Hidden Anagrams
// 2018.2.8 bal4u

use std::io::{self, BufRead};

const HASHSIZ: usize = 10007;

#[derive(Default)]
struct Hash {
    f: [i16; 26],
}

struct HashTable {
    table: [Hash; HASHSIZ + 2],
}

impl HashTable {
    fn new() -> Self {
        HashTable {
            table: [Hash::default(); HASHSIZ + 2], // Initialize the table with default Hash values
        }
    }

    fn insert(&mut self, f: &mut [i16; 26]) {
        f[0] += 1;
        let mut k: u64 = 0;
        for i in (0..26).step_by(2) {
            k = (k << 4) + f[i] as u64;
        }
        let mut tp = &mut self.table[(k % HASHSIZ as u64) as usize];
        while tp.f[0] != 0 {
            if tp.f == *f {
                return; // Found a match, exit
            }
            tp = if tp as *const _ == self.table.as_ptr().add(HASHSIZ) {
                &mut self.table[0]
            } else {
                unsafe { &mut *tp.add(1) }
            };
        }
        tp.f.copy_from_slice(f);
        f[0] -= 1;
    }

    fn lookup(&mut self, f: &mut [i16; 26]) -> bool {
        f[0] += 1;
        let mut k: u64 = 0;
        for i in (0..26).step_by(2) {
            k = (k << 4) + f[i] as u64;
        }
        let mut tp = &mut self.table[(k % HASHSIZ as u64) as usize];
        while tp.f[0] != 0 {
            if tp.f == *f {
                f[0] -= 1;
                return true; // Found a match
            }
            tp = if tp as *const _ == self.table.as_ptr().add(HASHSIZ) {
                &mut self.table[0]
            } else {
                unsafe { &mut *tp.add(1) }
            };
        }
        f[0] -= 1;
        false
    }
}

fn main() {
    let stdin = io::stdin();
    let mut sa = String::new();
    let mut sb = String::new();
    let mut f = [0i16; 26];

    stdin.lock().read_line(&mut sa).unwrap();
    let wa = sa.chars().take_while(|&c| c > ' ').count();
    let p: Vec<_> = sa.chars().take(wa).map(|c| (c as u8 - b'a') as usize).collect();
    for &index in &p {
        f[index] += 1;
    }

    stdin.lock().read_line(&mut sb).unwrap();
    let wb = sb.chars().take_while(|&c| c > ' ').count();
    let q: Vec<_> = sb.chars().take(wb).map(|c| (c as u8 - b'a') as usize).collect();
    for &index in &q {
        f[index] += 1;
    }

    let w = wa.min(wb);
    let mut hash_table = HashTable::new();

    for k in (1..=w).rev() {
        hash_table = HashTable::new();
        f = [0; 26];
        for &index in &p[..k] {
            f[index] += 1;
        }
        hash_table.insert(&mut f);

        let mut i = wa - k;
        while i > 0 {
            f[p[i] as usize] -= 1;
            f[p[i + k] as usize] += 1;
            hash_table.insert(&mut f);
            i -= 1;
        }

        f = [0; 26];
        for &index in &q[..k] {
            f[index] += 1;
        }
        if hash_table.lookup(&mut f) {
            println!("{}", k);
            return;
        }

        let mut i = wb - k;
        while i > 0 {
            f[q[i] as usize] -= 1;
            f[q[i + k] as usize] += 1;
            if hash_table.lookup(&mut f) {
                println!("{}", k);
                return;
            }
            i -= 1;
        }
    }
}
