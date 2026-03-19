use std::io::{self, Read};

#[derive(Clone)]
struct Dsu {
    parent: Vec<usize>,
    size: Vec<i64>,
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

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }
        let root = self.find(self.parent[x]);
        self.parent[x] = root;
        root
    }

    fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn comp_size(&mut self, x: usize) -> i64 {
        let r = self.find(x);
        self.size[r]
    }

    fn unite(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }
        // Union by size (keep semantics: component sizes maintained correctly)
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        true
    }
}

fn main() {
    // No explicit error handling per specification.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut edges: Vec<(usize, usize)> = Vec::with_capacity(m);
    for _ in 0..m {
        let a_in: usize = it.next().unwrap().parse().unwrap();
        let b_in: usize = it.next().unwrap().parse().unwrap();
        // Specification implies 1-based input indexing and shifting by 1.
        let a = a_in - 1;
        let b = b_in - 1;
        edges.push((a, b));
    }

    let mut dsu = Dsu::new(n);
    let mut ans: Vec<i64> = vec![0; m];

    // Initial disconnected-pair count: C(n,2)
    let mut disconnected: i64 = (n as i64) * ((n as i64) - 1) / 2;

    // Output mapping:
    // ans[i] corresponds to state after incorporating edges with indices i+1..m-1.
    // We build state by adding edges in reverse order.
    // Start with no edges added => corresponds to i = m-1.
    if m > 0 {
        ans[m - 1] = disconnected;
    }

    // For i from m-2 down to 0:
    // add edge i+1, then record disconnected as ans[i].
    // Note: Spec says behavior for m=1 is not well-defined by the original code;
    // here the loop naturally does nothing and prints one line.
    for i in (0..m.saturating_sub(1)).rev() {
        let (u, v) = edges[i + 1];
        if !dsu.same(u, v) {
            let su = dsu.comp_size(u);
            let sv = dsu.comp_size(v);
            disconnected -= su * sv;
            dsu.unite(u, v);
        }
        ans[i] = disconnected;
    }

    let mut out = String::new();
    for i in 0..m {
        out.push_str(&format!("{}\n", ans[i]));
    }
    print!("{out}");
}
