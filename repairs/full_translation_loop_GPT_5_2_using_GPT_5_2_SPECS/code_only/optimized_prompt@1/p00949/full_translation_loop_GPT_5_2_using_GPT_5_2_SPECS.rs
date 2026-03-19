use std::io::{self, Read};

const HASHSIZ: usize = 10007;

#[derive(Clone, Copy)]
struct HashEntry {
    f: [i16; 26],
}

fn insert(table: &mut [HashEntry; HASHSIZ + 2], f: &mut [i16; 26]) {
    // C code temporarily increments f[0] to ensure non-zero sentinel in hash slots.
    f[0] = f[0].wrapping_add(1);

    // Build key from even indices, using 4-bit shifts as in C.
    let mut k: u64 = 0;
    for i in (0..26).step_by(2) {
        k = (k << 4).wrapping_add(f[i] as u16 as u64);
    }

    let mut idx = (k % HASHSIZ as u64) as usize;
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

fn lookup(table: &[HashEntry; HASHSIZ + 2], f: &mut [i16; 26]) -> bool {
    f[0] = f[0].wrapping_add(1);

    let mut k: u64 = 0;
    for i in (0..26).step_by(2) {
        k = (k << 4).wrapping_add(f[i] as u16 as u64);
    }

    let mut idx = (k % HASHSIZ as u64) as usize;
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
    let mut it = input.split_inclusive('\n');

    let line_a = it.next().unwrap_or("");
    let line_b = it.next().unwrap_or("");

    // Convert in-place like C: while (*p > ' ') *p -= 'a'
    // We store only the prefix of bytes > ' ' (space), mapped to 0..25.
    fn parse_line(s: &str) -> Vec<u8> {
        let bytes = s.as_bytes();
        let mut out = Vec::new();
        for &ch in bytes {
            if ch > b' ' {
                out.push(ch.wrapping_sub(b'a'));
            } else {
                break;
            }
        }
        out
    }

    let sa = parse_line(line_a);
    let sb = parse_line(line_b);
    let wa = sa.len();
    let wb = sb.len();
    let w = wa.min(wb);

    let mut table = [HashEntry { f: [0i16; 26] }; HASHSIZ + 2];
    let mut f = [0i16; 26];

    let mut k_found: usize = 0;

    'outer: for k in (1..=w).rev() {
        // memset(hash, 0, sizeof(hash));
        for e in table.iter_mut() {
            e.f = [0i16; 26];
        }

        // Build all k-length window frequency vectors for sa and insert into hash.
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
            let idx_q = sa[q] as usize;
            f[idx_q] = f[idx_q].wrapping_sub(1);
            q += 1;

            let idx_p = sa[p] as usize;
            f[idx_p] = f[idx_p].wrapping_add(1);
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
            let idx_q = sb[q] as usize;
            f[idx_q] = f[idx_q].wrapping_sub(1);
            q += 1;

            let idx_p = sb[p] as usize;
            f[idx_p] = f[idx_p].wrapping_add(1);
            p += 1;

            if lookup(&table, &mut f) {
                k_found = k;
                break 'outer;
            }
            i -= 1;
        }
    }

    // C prints k after loop; if none found, k becomes 0 after loop ends.
    // Here we tracked k_found similarly.
    println!("{}", k_found);
}
