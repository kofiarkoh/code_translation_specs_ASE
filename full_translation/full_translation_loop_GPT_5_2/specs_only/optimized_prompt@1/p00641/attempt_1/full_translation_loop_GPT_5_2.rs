use std::io::{self, Read};

const MOD: i32 = 10007;

#[derive(Clone)]
struct DSU {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl DSU {
    fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n + 1);
        for i in 0..=n {
            parent.push(i);
        }
        Self {
            parent,
            rank: vec![0; n + 1],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let p = self.parent[x];
        if p != x {
            let r = self.find(p);
            self.parent[x] = r;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return;
        }
        let rka = self.rank[ra];
        let rkb = self.rank[rb];
        if rka < rkb {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        if rka == rkb {
            self.rank[ra] = rka.saturating_add(1);
        }
    }
}

fn main() {
    // Specification does not define exact input format beyond:
    // - an integer n (number of entities)
    // - followed by relationship pairs and an associated value
    //
    // Narrowest reasonable interpretation:
    // - Read all integers from stdin.
    // - First integer is n.
    // - Remaining integers are consumed in triples: (a, b, v).
    // - For each triple, union(a, b) and record v for that relationship.
    //
    // If trailing integers do not form a complete triple, they are ignored.

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap_or(0),
        None => return,
    };

    let mut dsu = DSU::new(n);

    // Store edges to process maxima after unions are complete.
    let mut edges: Vec<(usize, usize, i32)> = Vec::new();

    loop {
        let a = match it.next() {
            Some(s) => s.parse::<usize>().unwrap_or(0),
            None => break,
        };
        let b = match it.next() {
            Some(s) => s.parse::<usize>().unwrap_or(0),
            None => break,
        };
        let v = match it.next() {
            Some(s) => s.parse::<i32>().unwrap_or(0),
            None => break,
        };

        // Indices are assumed to be within 1..=n by precondition.
        // If not, we ignore out-of-range unions/edges to avoid panics.
        if a >= 1 && a <= n && b >= 1 && b <= n {
            dsu.union(a, b);
            edges.push((a, b, v));
        }
    }

    // Determine maximum value associated with each group.
    // The specification says "maximum values associated with each group".
    // We interpret "associated values" as the v values from relationships (edges)
    // whose endpoints are in that group.
    let mut maxv: Vec<i32> = vec![0; n + 1];
    for (a, _b, v) in edges {
        let r = dsu.find(a);
        if v > maxv[r] {
            maxv[r] = v;
        }
    }

    // Compute product of counts derived from maxima, modulo 10007.
    // The specification does not define the exact mapping from max value to count.
    // Narrowest reasonable interpretation consistent with "counts derived from the maximum values":
    // - For each group, contribute (max_value + 1) to the product.
    // This ensures non-negative output when max_value is non-negative.
    // If max_value is negative (not specified), treat its contribution as 0 (clamped),
    // preserving the postcondition of non-negative output.
    let mut ans: i32 = 1;
    for i in 1..=n {
        if dsu.find(i) == i {
            let m = maxv[i];
            let count = if m >= 0 { m.saturating_add(1) } else { 0 };
            ans = ((ans as i64 * (count as i64)) % (MOD as i64)) as i32;
        }
    }

    // Ensure output is within modulo range and non-negative.
    let out = ((ans % MOD) + MOD) % MOD;
    println!("{}", out);
}
