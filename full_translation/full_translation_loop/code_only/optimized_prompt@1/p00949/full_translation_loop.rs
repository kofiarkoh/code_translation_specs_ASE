// AOJ 1370 Hidden Anagrams
// 2018.2.8 bal4u

use std::io::{self, BufRead};

const HASHSIZ: usize = 10007;

#[derive(Default, Clone)]
struct Hash {
    f: [i16; 26],
}

struct HashTable {
    table: [Hash; HASHSIZ + 2],
}

impl HashTable {
    fn new() -> Self {
        HashTable {
            table: [Hash::default(); HASHSIZ + 2],
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
                f[0] -= 1; // Decrement before returning
                return; // Found a match, exit
            }
            tp = if tp as *const _ == self.table.as_ptr().add(HASHSIZ + 1) {
                &mut self.table[0]
            } else {
                unsafe { &mut *(tp as *const _ as *mut Hash).add(1) }
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
                f[0] -= 1; // Decrement before returning
                return true; // Found a match
            }
            tp = if tp as *const _ == self.table.as_ptr().add(HASHSIZ + 1) {
                &mut self.table[0]
            } else {
                unsafe { &mut *(tp as *const _ as *mut Hash).add(1) }
            };
        }
        f[0] -= 1;
        false
    }
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut sa = String::new();
    let mut sb = String::new();
    handle.read_line(&mut sa).unwrap();
    handle.read_line(&mut sb).unwrap();

    let wa = sa.trim().len() as i32;
    let wb = sb.trim().len() as i32;

    let mut f = [0i16; 26];

    let mut w = wa;
    if w > wb {
        w = wb;
    }

    let mut hash_table = HashTable::new();

    for k in (1..=w).rev() {
        // Reset hash table
        hash_table = HashTable::new();

        // Process first string
        f.iter_mut().for_each(|x| *x = 0);
        for c in sa.chars().take(k as usize) {
            f[c as usize - 'a' as usize] += 1;
        }
        hash_table.insert(&mut f);

        let mut i = wa - k;
        let mut p = sa.chars();
        while i > 0 {
            f[p.next().unwrap() as usize - 'a' as usize] -= 1;
            f[p.next().unwrap() as usize - 'a' as usize] += 1;
            hash_table.insert(&mut f);
            i -= 1;
        }

        // Process second string
        f.iter_mut().for_each(|x| *x = 0);
        for c in sb.chars().take(k as usize) {
            f[c as usize - 'a' as usize] += 1;
        }
        if hash_table.lookup(&mut f) {
            println!("{}", k);
            return;
        }

        let mut i = wb - k;
        let mut q = sb.chars();
        while i > 0 {
            f[q.next().unwrap() as usize - 'a' as usize] -= 1;
            f[q.next().unwrap() as usize - 'a' as usize] += 1;
            if hash_table.lookup(&mut f) {
                println!("{}", k);
                return;
            }
            i -= 1;
        }
    }
}
