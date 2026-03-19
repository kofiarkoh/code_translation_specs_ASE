use std::io::{self, Read};

const HASHSIZ: usize = 10007;

#[derive(Clone, Copy)]
struct HashEntry {
    f: [i16; 26],
}

fn insert(table: &mut [HashEntry], f: &mut [i16; 26]) {
    // Emulate C: f[0]++ used as a sentinel to ensure f[0] != 0 for occupied entries.
    f[0] = f[0].wrapping_add(1);

    // C uses unsigned long long and shifts by 4, adding f[i] (short) for i=0,2,...,24.
    // Use u64 and wrap like C unsigned arithmetic.
    let mut k: u64 = 0;
    for i in (0..26).step_by(2) {
        k = (k << 4).wrapping_add(f[i] as i64 as u64);
    }

    let mut idx = (k % (HASHSIZ as u64)) as usize;
    loop {
        if table[idx].f[0] == 0 {
            table[idx].f = *f;
            break;
        }
        if table[idx].f == *f {
            break;
        }
        idx += 1;
        if idx == HASHSIZ {
            idx = 0;
        }
    }

    f[0] = f[0].wrapping_sub(1);
}

fn lookup(table: &[HashEntry], f: &mut [i16; 26]) -> bool {
    f[0] = f[0].wrapping_add(1);

    let mut k: u64 = 0;
    for i in (0..26).step_by(2) {
        k = (k << 4).wrapping_add(f[i] as i64 as u64);
    }

    let mut idx = (k % (HASHSIZ as u64)) as usize;
    loop {
        if table[idx].f[0] == 0 {
            f[0] = f[0].wrapping_sub(1);
            return false;
        }
        if table[idx].f == *f {
            f[0] = f[0].wrapping_sub(1);
            return true;
        }
        idx += 1;
        if idx == HASHSIZ {
            idx = 0;
        }
    }
}

fn main() {
    // Read entire stdin; C uses fgets twice.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.lines();

    let line_a = it.next().unwrap_or("");
    let line_b = it.next().unwrap_or("");

    // C: while (*p > ' ') { *p -= 'a'; p++; } wa = p - sa;
    // So it processes until first whitespace (<= ' '), converting 'a'.. to 0.. by subtraction.
    let mut sa: Vec<u8> = Vec::new();
    for &b in line_a.as_bytes() {
        if b <= b' ' {
            break;
        }
        sa.push(b.wrapping_sub(b'a'));
    }
    let wa = sa.len();

    let mut sb: Vec<u8> = Vec::new();
    for &b in line_b.as_bytes() {
        if b <= b' ' {
            break;
        }
        sb.push(b.wrapping_sub(b'a'));
    }
    let wb = sb.len();

    let w = wa.min(wb);

    let mut table = vec![HashEntry { f: [0i16; 26] }; HASHSIZ + 2];
    let mut f = [0i16; 26];

    let mut k_found: usize = 0;

    'outer: for k in (1..=w).rev() {
        // memset(hash, 0, sizeof(hash));
        for e in table.iter_mut() {
            e.f = [0i16; 26];
        }

        // Build all length-k window histograms for sa and insert into hash.
        f = [0i16; 26];
        let mut p = 0usize;
        for _ in 0..k {
            let idx = sa[p] as usize;
            f[idx] = f[idx].wrapping_add(1);
            p += 1;
        }
        insert(&mut table, &mut f);

        let mut q = 0usize;
        let mut i = wa - k;
        while i > 0 {
            let out = sa[q] as usize;
            let inn = sa[p] as usize;
            f[out] = f[out].wrapping_sub(1);
            f[inn] = f[inn].wrapping_add(1);
            q += 1;
            p += 1;
            insert(&mut table, &mut f);
            i -= 1;
        }

        // Slide over sb and lookup.
        f = [0i16; 26];
        p = 0;
        for _ in 0..k {
            let idx = sb[p] as usize;
            f[idx] = f[idx].wrapping_add(1);
            p += 1;
        }
        if lookup(&table, &mut f) {
            k_found = k;
            break 'outer;
        }

        q = 0;
        i = wb - k;
        while i > 0 {
            let out = sb[q] as usize;
            let inn = sb[p] as usize;
            f[out] = f[out].wrapping_sub(1);
            f[inn] = f[inn].wrapping_add(1);
            q += 1;
            p += 1;
            if lookup(&table, &mut f) {
                k_found = k;
                break 'outer;
            }
            i -= 1;
        }
    }

    // If no match found, C prints k after loop ends (k==0). We tracked k_found accordingly.
    println!("{}", k_found);
}
