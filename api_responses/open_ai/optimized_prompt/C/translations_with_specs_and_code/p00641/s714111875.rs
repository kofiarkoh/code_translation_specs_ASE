// AOJ 1055 Huge Family
// Translated from C to Rust

use std::io::{self, BufRead};

const MAX: usize = 100002;

struct UnionFind {
    id: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut id = Vec::with_capacity(n);
        let mut size = Vec::with_capacity(n);
        for i in 0..n {
            id.push(i);
            size.push(1);
        }
        UnionFind { id, size }
    }

    fn root(&mut self, i: usize) -> usize {
        if i != self.id[i] {
            self.id[i] = self.root(self.id[i]); // Path compression
        }
        self.id[i]
    }

    fn connected(&mut self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }

    fn unite(&mut self, p: usize, q: usize) {
        let i = self.root(p);
        let j = self.root(q);
        if i == j {
            return;
        }
        if self.size[i] < self.size[j] {
            self.id[i] = j;
            self.size[j] += self.size[i];
        } else {
            self.id[j] = i;
            self.size[i] += self.size[j];
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut input = reader.lines().filter_map(Result::ok);
    
    let mut f = vec![(0, 0); MAX];
    let mut max = vec![0; MAX];
    let mut cnt = vec![0; MAX];

    while let Some(line) = input.next() {
        let n: usize = line.trim().parse().unwrap();
        let mut uf = UnionFind::new(n);

        for i in 0..n {
            let line = input.next().unwrap();
            let parts: Vec<usize> = line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let a = parts[0];
            let b = parts[1];
            f[i] = (a, b);
            uf.unite(i, a);
            uf.unite(i, b);
        }

        cnt.fill(0); // Reset cnt array
        for i in 0..n {
            let k = uf.root(i);
            check(k, f[i].0, &mut cnt, &mut max);
            check(k, f[i].1, &mut cnt, &mut max);
        }

        let mut ans = 1;
        for i in 0..n {
            if cnt[i] > 0 {
                ans = ans * (cnt[i] / 2) % 10007;
            }
        }
        println!("{}", ans);
    }
}

fn check(k: usize, f: usize, cnt: &mut Vec<usize>, max: &mut Vec<usize>) {
    if cnt[k] == 0 || f > max[k] {
        cnt[k] = 1;
        max[k] = f;
    } else if f == max[k] {
        cnt[k] += 1;
    }
}
```

### Notes:
1. **Union-Find Structure**: The `UnionFind` struct encapsulates the union-find logic, maintaining the `id` and `size` arrays.
2. **Input Handling**: The input is read using `BufRead` for efficiency, and the parsing is done using `split_whitespace` to handle multiple integers in a line.
3. **Array Initialization**: The `cnt` array is reset using `fill(0)` to ensure it starts from a clean state for each new set of relationships.
4. **Check Function**: The `check` function is defined outside of `main` to maintain clarity and separation of concerns.
5. **Output**: The output is printed in the same format as the original C code, ensuring that the behavior remains consistent.
