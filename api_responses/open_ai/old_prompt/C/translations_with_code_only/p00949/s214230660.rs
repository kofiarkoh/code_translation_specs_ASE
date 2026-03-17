// AOJ 1370 Hidden Anagrams
// 2018.2.8 bal4u

use std::io::{self, BufRead};
use std::mem;

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
            table: Default::default(),
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
                return; // Already exists
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
                f[0] -= 1;
                return true;
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

    let mut sa = vec![0; 4004];
    let mut sb = vec![0; 4004];
    let mut f = [0; 26];
    let mut wa = 0;
    let mut wb = 0;

    // Read first string
    let bytes = handle.read_line(&mut sa).unwrap();
    let p = sa.iter_mut().take(bytes).filter(|&&mut c| c > b' ').collect::<Vec<_>>();
    for c in p {
        *c -= b'a';
        wa += 1;
    }

    // Read second string
    let bytes = handle.read_line(&mut sb).unwrap();
    let p = sb.iter_mut().take(bytes).filter(|&&mut c| c > b' ').collect::<Vec<_>>();
    for c in p {
        *c -= b'a';
        wb += 1;
    }

    let mut w = wa;
    if w > wb {
        w = wb;
    }

    let mut hash_table = HashTable::new();

    for k in (1..=w).rev() {
        // Reset hash table
        hash_table = HashTable::new();

        // Insert first k characters from sa
        f = [0; 26];
        let mut p = &sa[..k];
        for &c in p {
            f[c as usize] += 1;
        }
        hash_table.insert(&mut f);

        // Sliding window for sa
        let mut i = wa - k;
        let mut q = &sa[..];
        while i > 0 {
            f[q[i] as usize] -= 1;
            f[p[i] as usize] += 1;
            hash_table.insert(&mut f);
            i -= 1;
        }

        // Check against sb
        f = [0; 26];
        p = &sb[..k];
        for &c in p {
            f[c as usize] += 1;
        }
        if hash_table.lookup(&mut f) {
            println!("{}", k);
            return;
        }

        // Sliding window for sb
        i = wb - k;
        q = &sb[..];
        while i > 0 {
            f[q[i] as usize] -= 1;
            f[p[i] as usize] += 1;
            if hash_table.lookup(&mut f) {
                println!("{}", k);
                return;
            }
            i -= 1;
        }
    }
}
