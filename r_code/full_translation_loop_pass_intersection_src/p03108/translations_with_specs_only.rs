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

    fn unite(&mut self, a: usize, b: usize) -> i64 {
        // Returns the decrease in disconnected-pair count:
        // if merged, decrease = size(ra) * size(rb), else 0.
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return 0;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        let dec = self.size[ra] * self.size[rb];
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        dec
    }
}

fn main() {
    // No explicit error handling per specification; malformed input => undefined behavior.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    // Specification assumes m >= 1; behavior for m == 0 is not supported.
    let mut edges: Vec<(usize, usize)> = Vec::with_capacity(m);
    for _ in 0..m {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        // Input is 1-based; convert to 0-based.
        edges.push((a - 1, b - 1));
    }

    let mut dsu = Dsu::new(n);

    // Initial disconnected pairs = C(n,2).
    let mut disconnected: i64 = (n as i64) * ((n as i64) - 1) / 2;

    // ans[i] corresponds to graph with edges j > i present.
    // Process edges in reverse, storing the count *before* adding edge i.
    let mut ans: Vec<i64> = vec![0; m];
    for i in (0..m).rev() {
        ans[i] = disconnected;
        let (a, b) = edges[i];
        let dec = dsu.unite(a, b);
        disconnected -= dec;
    }

    let mut out = String::new();
    for i in 0..m {
        out.push_str(&format!("{}\n", ans[i]));
    }
    print!("{out}");
}
