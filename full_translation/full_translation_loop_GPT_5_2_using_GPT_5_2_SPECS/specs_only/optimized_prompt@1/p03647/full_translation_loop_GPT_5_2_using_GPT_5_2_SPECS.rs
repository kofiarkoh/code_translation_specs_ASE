use std::io::{self, Read};

struct DsuParity {
    parent: Vec<usize>,
    rank: Vec<u8>,
    // parity[x] = xor from x to parent[x]
    parity: Vec<u8>,
}

impl DsuParity {
    fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        for i in 0..n {
            parent.push(i);
        }
        Self {
            parent,
            rank: vec![0; n],
            parity: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> (usize, u8) {
        if self.parent[x] == x {
            return (x, 0);
        }
        let p = self.parent[x];
        let (r, pr) = self.find(p);
        // Path compression while preserving parity-to-root.
        self.parent[x] = r;
        self.parity[x] ^= pr;
        (self.parent[x], self.parity[x])
    }

    // Enforce: value(a) xor value(b) == w, where w is 0 (same) or 1 (different).
    // Returns false if contradiction detected.
    fn unite(&mut self, a: usize, b: usize, w: u8) -> bool {
        let (ra, pa) = self.find(a);
        let (rb, pb) = self.find(b);

        if ra == rb {
            return (pa ^ pb) == w;
        }

        // Attach lower-rank root under higher-rank root.
        let mut ra = ra;
        let mut rb = rb;
        let mut pa = pa;
        let mut pb = pb;

        if self.rank[ra] < self.rank[rb] {
            std::mem::swap(&mut ra, &mut rb);
            std::mem::swap(&mut pa, &mut pb);
        }

        self.parent[rb] = ra;
        // Need parity[rb] such that:
        // (value(b) xor value(rb)) xor (value(a) xor value(ra)) == w
        // With pa = value(a) xor value(ra), pb = value(b) xor value(rb)
        // After linking rb -> ra, set value(rb) xor value(ra) = pa ^ pb ^ w
        self.parity[rb] = pa ^ pb ^ w;

        if self.rank[ra] == self.rank[rb] {
            self.rank[ra] = self.rank[ra].saturating_add(1);
        }
        true
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap_or(0),
        None => {
            // No items count provided; behavior not inferable. Choose narrowest:
            // treat as no constraints => POSSIBLE.
            println!("POSSIBLE");
            return;
        }
    };

    // Fixed maximum capacity per specification; behavior beyond is not inferable.
    // We still allocate based on n as given.
    let mut dsu = DsuParity::new(n);

    let mut possible = true;

    // Read constraints as pairs of indices. Any trailing unpaired integer is ignored.
    while let (Some(a_s), Some(b_s)) = (it.next(), it.next()) {
        if !possible {
            // Once contradiction detected, final result remains IMPOSSIBLE.
            // Still consume input (already doing so) but do not change state.
            continue;
        }

        let a: usize = match a_s.parse() {
            Ok(v) => v,
            Err(_) => continue, // input well-formed per spec; keep narrow behavior
        };
        let b: usize = match b_s.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        // Specification does not define how to distinguish "same" vs "different" from input.
        // Narrowest reasonable interpretation: each pair enforces "different".
        // This still matches the described model (binary constraints) without inventing extra fields.
        let w: u8 = 1;

        // Preconditions say indices are valid; behavior otherwise not inferable.
        if a < n && b < n {
            if !dsu.unite(a, b, w) {
                possible = false;
            }
        } else {
            // Out-of-range behavior not inferable; do nothing.
        }
    }

    if possible {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}
