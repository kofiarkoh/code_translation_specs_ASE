use std::collections::HashSet;
use std::hash::{BuildHasherDefault, Hasher};
use std::io::{self, Read};

fn reverse_range(bytes: &mut [u8], start: usize, len: usize) {
    let mut i = 0usize;
    while i < len / 2 {
        bytes.swap(start + i, start + (len - 1 - i));
        i += 1;
    }
}

/// C `swap(s1, s2)` where `s2` points inside `s1`:
/// result becomes `s2..end` followed by `s1..s2`.
#[inline]
fn rotate_at(bytes: &mut [u8], split: usize) {
    bytes.rotate_left(split);
}

/// Very fast non-cryptographic hasher (FNV-1a 64-bit).
/// This is safe and deterministic; used only for performance to avoid timeouts.
#[derive(Default)]
struct Fnv1a64(u64);
impl Hasher for Fnv1a64 {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let mut h = self.0;
        if h == 0 {
            h = 0xcbf29ce484222325;
        }
        for &b in bytes {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        self.0 = h;
    }
    #[inline]
    fn finish(&self) -> u64 {
        if self.0 == 0 {
            0xcbf29ce484222325
        } else {
            self.0
        }
    }
}

type FastBuildHasher = BuildHasherDefault<Fnv1a64>;

fn main() {
    // Read all input, whitespace-delimited.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut out = String::new();

    for _ in 0..t {
        let s = it.next().unwrap_or("");
        let orig = s.as_bytes().to_vec();
        let len = orig.len();

        // SPEC: If L <= 1, output 0.
        if len <= 1 {
            out.push_str("0\n");
            continue;
        }

        // Store unique results. Spec guarantees <= 1024.
        let mut set: HashSet<Vec<u8>, FastBuildHasher> =
            HashSet::with_capacity_and_hasher(1024, FastBuildHasher::default());

        for i in 1..len {
            // Sequence 1
            let mut cur = orig.clone();
            set.insert(cur.clone());
            reverse_range(&mut cur, 0, i);
            set.insert(cur.clone());
            reverse_range(&mut cur, i, len - i);
            set.insert(cur.clone());
            reverse_range(&mut cur, 0, i);
            set.insert(cur);

            // Sequence 2
            let mut cur = orig.clone();
            rotate_at(&mut cur, i);
            set.insert(cur.clone());
            reverse_range(&mut cur, 0, len - i);
            set.insert(cur.clone());
            reverse_range(&mut cur, len - i, i);
            set.insert(cur.clone());
            reverse_range(&mut cur, 0, len - i);
            set.insert(cur);
        }

        out.push_str(&format!("{}\n", set.len()));
    }

    print!("{out}");
}
