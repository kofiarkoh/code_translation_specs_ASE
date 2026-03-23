use std::io::{self, Read};

const HASHSIZ: usize = 10007;

#[derive(Clone, Copy)]
struct HashEntry {
    f: [i16; 26],
}

fn insert(table: &mut [HashEntry], f: &mut [i16; 26]) {
    // Emulate C: f[0]++ used as a sentinel to ensure f[0] is non-zero in table.
    f[0] = f[0].wrapping_add(1);

    let mut k: u64 = 0;
    for i in (0..26).step_by(2) {
        // C does: k = (k << 4) + f[i]; with f[i] a signed short promoted to unsigned long long.
        // That means negative values would be converted to a huge u64. We emulate that by
        // sign-extending to i64 then casting to u64.
        let add = (f[i] as i64) as u64;
        k = (k << 4).wrapping_add(add);
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
        let add = (f[i] as i64) as u64;
        k = (k << 4).wrapping_add(add);
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

fn read_line_bytes(input: &mut &[u8]) -> Vec<u8> {
    if input.is_empty() {
        return Vec::new();
    }
    if let Some(pos) = input.iter().position(|&b| b == b'\n') {
        let line = input[..=pos].to_vec();
        *input = &input[pos + 1..];
        line
    } else {
        let line = input.to_vec();
        *input = &[];
        line
    }
}

fn main() {
    // Read entire stdin, then extract two lines like fgets would.
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut bytes: &[u8] = buf.as_bytes();

    let mut sa = read_line_bytes(&mut bytes);
    let mut sb = read_line_bytes(&mut bytes);

    // C: while (*p > ' ') *p -= 'a', p++; wa = p - sa;
    // This mutates bytes in-place to 0..25 values until first <= ' '.
    let mut wa: usize = 0;
    while wa < sa.len() && sa[wa] > b' ' {
        sa[wa] = sa[wa].wrapping_sub(b'a');
        wa += 1;
    }

    let mut wb: usize = 0;
    while wb < sb.len() && sb[wb] > b' ' {
        sb[wb] = sb[wb].wrapping_sub(b'a');
        wb += 1;
    }

    let mut w = wa;
    if w > wb {
        w = wb;
    }

    let mut table = vec![HashEntry { f: [0i16; 26] }; HASHSIZ + 2];
    let mut f = [0i16; 26];

    let mut k_out: i32 = 0;

    'outer: for k in (1..=w).rev() {
        // memset(hash, 0, sizeof(hash));
        for e in table.iter_mut() {
            e.f = [0i16; 26];
        }

        // Build all k-length window histograms for sa and insert.
        f = [0i16; 26];
        let mut p = 0usize;
        for _ in 0..k {
            let idx = sa[p] as usize;
            f[idx] = f[idx].wrapping_add(1);
            p += 1;
        }
        insert(&mut table, &mut f);

        let mut i = wa - k;
        let mut q = 0usize;
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

        // Now scan sb windows and lookup.
        f = [0i16; 26];
        p = 0;
        for _ in 0..k {
            let idx = sb[p] as usize;
            f[idx] = f[idx].wrapping_add(1);
            p += 1;
        }
        if lookup(&table, &mut f) {
            k_out = k as i32;
            break 'outer;
        }

        i = wb - k;
        q = 0;
        while i > 0 {
            let idx_q = sb[q] as usize;
            f[idx_q] = f[idx_q].wrapping_sub(1);
            q += 1;

            let idx_p = sb[p] as usize;
            f[idx_p] = f[idx_p].wrapping_add(1);
            p += 1;

            if lookup(&table, &mut f) {
                k_out = k as i32;
                break 'outer;
            }
            i -= 1;
        }
    }

    // If no match found, C prints k after loop ends (k==0). We tracked k_out accordingly.
    // When no match, k_out remains 0.
    print!("{}\n", k_out);
}
