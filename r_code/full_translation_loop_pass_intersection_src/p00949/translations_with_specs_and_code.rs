use std::io::{self, Read};

const HASHSIZ: usize = 10007;

#[derive(Clone, Copy)]
struct Entry {
    f: [i16; 26], // f[0] == 0 means empty slot (as in the C code)
}

fn hash_key(f: &[i16; 26]) -> usize {
    // Mirrors C:
    // f[0]++ (temporarily), then k = 0; for i=0; i<26; i+=2: k = (k<<4) + f[i]
    // then k % HASHSIZ
    let mut k: u64 = 0;
    for i in (0..26).step_by(2) {
        k = (k << 4) + (f[i] as u64);
    }
    (k % (HASHSIZ as u64)) as usize
}

fn insert(table: &mut [Entry; HASHSIZ], f: &mut [i16; 26]) {
    // Use f[0] as a temporary non-zero marker to match C's empty-slot test.
    f[0] += 1;
    let mut idx = hash_key(f);

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

    f[0] -= 1;
}

fn lookup(table: &[Entry; HASHSIZ], f: &mut [i16; 26]) -> bool {
    f[0] += 1;
    let mut idx = hash_key(f);

    loop {
        if table[idx].f[0] == 0 {
            f[0] -= 1;
            return false;
        }
        if table[idx].f == *f {
            f[0] -= 1;
            return true;
        }
        idx += 1;
        if idx == HASHSIZ {
            idx = 0;
        }
    }
}

fn parse_prefix_lowercase_as_0_25(line: &str) -> Vec<u8> {
    // Spec: string is prefix up to first whitespace; lowercase a-z.
    // C code stops at <= ' ' and maps by subtracting 'a'.
    let mut v = Vec::new();
    for b in line.bytes() {
        if b <= b' ' {
            break;
        }
        v.push(b - b'a');
    }
    v
}

fn main() {
    // Read entire stdin; take first two lines (or empty if missing, per "no explicit error handling").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.lines();

    let sa_line = it.next().unwrap_or("");
    let sb_line = it.next().unwrap_or("");

    let sa = parse_prefix_lowercase_as_0_25(sa_line);
    let sb = parse_prefix_lowercase_as_0_25(sb_line);

    let wa = sa.len();
    let wb = sb.len();
    let mut w = wa.min(wb);

    let mut table = [Entry { f: [0i16; 26] }; HASHSIZ];
    let mut f = [0i16; 26];

    let mut k_found = 0usize;

    // Try lengths from max down to 1; if none, answer is 0.
    while w > 0 {
        // Clear hash table
        table = [Entry { f: [0i16; 26] }; HASHSIZ];

        // Insert all length-w signatures from sa
        f = [0i16; 26];
        for &ch in &sa[0..w] {
            f[ch as usize] += 1;
        }
        insert(&mut table, &mut f);

        for start in 1..=wa.saturating_sub(w) {
            let out_ch = sa[start - 1] as usize;
            let in_ch = sa[start + w - 1] as usize;
            f[out_ch] -= 1;
            f[in_ch] += 1;
            insert(&mut table, &mut f);
        }

        // Check all length-w signatures from sb
        f = [0i16; 26];
        for &ch in &sb[0..w] {
            f[ch as usize] += 1;
        }
        if lookup(&table, &mut f) {
            k_found = w;
            break;
        }

        for start in 1..=wb.saturating_sub(w) {
            let out_ch = sb[start - 1] as usize;
            let in_ch = sb[start + w - 1] as usize;
            f[out_ch] -= 1;
            f[in_ch] += 1;
            if lookup(&table, &mut f) {
                k_found = w;
                break;
            }
        }
        if k_found != 0 {
            break;
        }

        w -= 1;
    }

    // If no match found, k_found remains 0, matching the C behavior (k ends at 0).
    print!("{k_found}\n");
}
