use std::io::{self, BufRead};

const HASHSIZ: usize = 10007;

#[derive(Default)]
struct Hash {
    f: [i16; 26],
}

fn insert(hash_table: &mut [Hash; HASHSIZ], f: &mut [i16; 26]) {
    f[0] += 1;
    let mut k: u64 = 0;
    for i in (0..26).step_by(2) {
        k = (k << 4) + f[i as usize] as u64;
    }
    let mut tp_index = (k % HASHSIZ as u64) as usize;

    while hash_table[tp_index].f[0] != 0 {
        if hash_table[tp_index].f == *f {
            return;
        }
        tp_index = (tp_index + 1) % HASHSIZ;
    }
    hash_table[tp_index].f.copy_from_slice(f);
    f[0] -= 1;
}

fn lookup(hash_table: &[Hash; HASHSIZ], f: &mut [i16; 26]) -> bool {
    f[0] += 1;
    let mut k: u64 = 0;
    for i in (0..26).step_by(2) {
        k = (k << 4) + f[i as usize] as u64;
    }
    let mut tp_index = (k % HASHSIZ as u64) as usize;

    while hash_table[tp_index].f[0] != 0 {
        if hash_table[tp_index].f == *f {
            f[0] -= 1;
            return true;
        }
        tp_index = (tp_index + 1) % HASHSIZ;
    }
    f[0] -= 1;
    false
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let sa = lines.next().unwrap().unwrap();
    let sb = lines.next().unwrap().unwrap();

    let mut hash_table: [Hash; HASHSIZ] = Default::default();
    let mut wa = sa.len();
    let mut wb = sb.len();
    let mut f = [0; 26];

    let mut w = wa;
    if w > wb {
        w = wb;
    }

    for k in (1..=w).rev() {
        hash_table.iter_mut().for_each(|h| h.f = [0; 26]);

        f = [0; 26];
        for c in sa.chars().take(k) {
            f[c as usize - 'a' as usize] += 1;
        }
        insert(&mut hash_table, &mut f);

        let mut i = wa - k;
        let mut p = sa.chars();
        for _ in 0..i {
            f[p.next().unwrap() as usize - 'a' as usize] -= 1;
            f[p.next().unwrap() as usize - 'a' as usize] += 1;
            insert(&mut hash_table, &mut f);
        }

        f = [0; 26];
        for c in sb.chars().take(k) {
            f[c as usize - 'a' as usize] += 1;
        }
        if lookup(&hash_table, &mut f) {
            println!("{}", k);
            return;
        }

        let mut i = wb - k;
        let mut q = sb.chars();
        for _ in 0..i {
            f[q.next().unwrap() as usize - 'a' as usize] -= 1;
            f[q.next().unwrap() as usize - 'a' as usize] += 1;
            if lookup(&hash_table, &mut f) {
                println!("{}", k);
                return;
            }
        }
    }
    println!("0");
}
