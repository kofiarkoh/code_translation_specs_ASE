use std::io::{self, Read};

#[derive(Clone)]
struct Dsu {
    parent: Vec<usize>,
    size: Vec<i32>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        let mut size = Vec::with_capacity(n);
        for i in 0..n {
            parent.push(i);
            size.push(1);
        }
        Self { parent, size }
    }

    fn find(&mut self, mut x: usize) -> usize {
        // Specification constraint: parent-chain depth encountered during queries does not exceed 20.
        // We emulate the fixed internal storage by using a fixed-size stack and panicking if exceeded.
        // (Spec says behavior is undefined; panic is a narrow, explicit failure mode.)
        let mut path = [0usize; 20];
        let mut len = 0usize;

        while self.parent[x] != x {
            if len >= 20 {
                panic!("parent-chain depth exceeded fixed bound (20)");
            }
            path[len] = x;
            len += 1;
            x = self.parent[x];
        }
        for i in 0..len {
            self.parent[path[i]] = x;
        }
        x
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return;
        }
        // Union by size to help keep depth small (still subject to the spec's depth precondition).
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
    }

    fn comp_size(&mut self, x: usize) -> i32 {
        let r = self.find(x);
        self.size[r]
    }
}

fn main() {
    // Read all input as whitespace-separated integers.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();
    let k: usize = it.next().unwrap().parse().unwrap();

    let mut dsu = Dsu::new(n);

    // Exclusion count starts at 1 for each entity (excluding itself).
    // Use i32: outputs are within [-something, n], and n <= 100000.
    let mut excl = vec![1i32; n];

    // Store edges to process unions first, then compute component membership for disallowed pairs.
    let mut edges: Vec<(usize, usize)> = Vec::with_capacity(m);

    for _ in 0..m {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        let ai = a - 1;
        let bi = b - 1;
        edges.push((ai, bi));
        // Each occurrence counts separately in exclusions.
        excl[ai] += 1;
        excl[bi] += 1;
    }

    // Build components from the m connections.
    for (a, b) in edges {
        dsu.union(a, b);
    }

    // Process disallowed pairs: only count if both endpoints are in the same component.
    for _ in 0..k {
        let c: usize = it.next().unwrap().parse().unwrap();
        let d: usize = it.next().unwrap().parse().unwrap();
        let ci = c - 1;
        let di = d - 1;
        if dsu.find(ci) == dsu.find(di) {
            // Each occurrence counts separately in exclusions.
            excl[ci] += 1;
            excl[di] += 1;
        }
    }

    // Output for each i:
    // (component size) - (exclusion count for i)
    // where exclusion count = 1 + degree(i) + disallowed_same_component_incident(i)
    let mut out = String::new();
    for i in 0..n {
        let ans = dsu.comp_size(i) - excl[i];
        if i > 0 {
            out.push(' ');
        }
        out.push_str(&ans.to_string());
    }
    out.push('\n');
    print!("{out}");
}
