use std::collections::HashMap;
use std::io::{self, BufRead};

const HASHSIZ: usize = 10007;

#[derive(Default, Clone)]
struct Hash {
    f: [i16; 26],
}

struct AnagramFinder {
    hash: Vec<Hash>,
}

impl AnagramFinder {
    fn new() -> Self {
        AnagramFinder {
            hash: vec![Hash::default(); HASHSIZ + 2],
        }
    }

    fn insert(&mut self, f: &mut [i16; 26]) {
        f[0] += 1;
        let mut k: u64 = 0;
        for i in (0..26).step_by(2) {
            k = (k << 4) + f[i as usize] as u64;
        }
        let mut tp_index = (k % HASHSIZ as u64) as usize;
        while self.hash[tp_index].f[0] != 0 {
            if self.hash[tp_index].f == *f {
                f[0] -= 1;
                return; // Found a match, no need to insert
            }
            tp_index = (tp_index + 1) % self.hash.len();
        }
        self.hash[tp_index].f.copy_from_slice(f);
        f[0] -= 1;
    }

    fn lookup(&mut self, f: &mut [i16; 26]) -> bool {
        f[0] += 1;
        let mut k: u64 = 0;
        for i in (0..26).step_by(2) {
            k = (k << 4) + f[i as usize] as u64;
        }
        let mut tp_index = (k % HASHSIZ as u64) as usize;
        while self.hash[tp_index].f[0] != 0 {
            if self.hash[tp_index].f == *f {
                f[0] -= 1;
                return true; // Found a match
            }
            tp_index = (tp_index + 1) % self.hash.len();
        }
        f[0] -= 1;
        false
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    let sa = input.next().unwrap().unwrap();
    let sb = input.next().unwrap().unwrap();

    let wa = sa.len();
    let wb = sb.len();
    let w = wa.min(wb);

    let mut finder = AnagramFinder::new();
    let mut f = [0; 26];

    for k in (1..=w).rev() {
        finder.hash.iter_mut().for_each(|h| h.f = [0; 26]); // Reset hash table

        f.iter_mut().for_each(|x| *x = 0); // Reset frequency array
        for c in sa.chars().take(k) {
            f[c as usize - 'a' as usize] += 1;
        }
        finder.insert(&mut f);

        let i = wa - k;
        let mut p = sa.chars();
        for _ in 0..i {
            f[p.next().unwrap() as usize - 'a' as usize] -= 1;
            f[p.next().unwrap() as usize - 'a' as usize] += 1;
            finder.insert(&mut f);
        }

        f.iter_mut().for_each(|x| *x = 0); // Reset frequency array
        for c in sb.chars().take(k) {
            f[c as usize - 'a' as usize] += 1;
        }
        if finder.lookup(&mut f) {
            println!("{}", k);
            return;
        }

        let i = wb - k;
        let mut q = sb.chars();
        for _ in 0..i {
            f[q.next().unwrap() as usize - 'a' as usize] -= 1;
            f[q.next().unwrap() as usize - 'a' as usize] += 1;
            if finder.lookup(&mut f) {
                println!("{}", k);
                return;
            }
        }
    }
}
