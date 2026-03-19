use std::io::{self, Read};

#[derive(Clone)]
struct DSU {
    parent: Vec<usize>,
    size: Vec<i64>,
}

impl DSU {
    fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n + 1);
        let mut size = Vec::with_capacity(n + 1);
        parent.push(0);
        size.push(0);
        for i in 1..=n {
            parent.push(i);
            size.push(1);
        }
        Self { parent, size }
    }

    fn find(&mut self, x: usize) -> usize {
        let p = self.parent[x];
        if p == x {
            x
        } else {
            let r = self.find(p);
            self.parent[x] = r;
            r
        }
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return;
        }
        // Union by size to keep structure shallow.
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
    }

    fn set_size_of(&mut self, x: usize) -> i64 {
        let r = self.find(x);
        self.size[r]
    }
}

fn main() {
    // Specification is ambiguous about exact query semantics; narrowest interpretation:
    // - Read n, m, q.
    // - Apply m unions.
    // - For each of q queries (u, v), if u and v are connected at query time, count one "removal"
    //   for u and one for v.
    // - Output for each element i: size(set(i)) - removals[i], after all operations.
    //
    // This preserves:
    // - Union-find invariants and size tracking.
    // - "adjusted for the number of removals associated with that element".
    // - Query operations that "check for connectivity".
    //
    // If inputs are out of range, behavior is not specified; we assume valid 1..=n indices.

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let m: usize = it.next().unwrap().parse().unwrap();
    let q: usize = it.next().unwrap().parse().unwrap();

    let mut dsu = DSU::new(n);

    for _ in 0..m {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        dsu.union(a, b);
    }

    let mut removals = vec![0i64; n + 1];

    for _ in 0..q {
        let u: usize = it.next().unwrap().parse().unwrap();
        let v: usize = it.next().unwrap().parse().unwrap();
        let ru = dsu.find(u);
        let rv = dsu.find(v);
        if ru == rv {
            removals[u] += 1;
            removals[v] += 1;
        }
    }

    let mut out = String::new();
    for i in 1..=n {
        let sz = dsu.set_size_of(i);
        let adjusted = sz - removals[i];
        out.push_str(&adjusted.to_string());
        if i != n {
            out.push(' ');
        }
    }
    out.push('\n');
    print!("{out}");
}
