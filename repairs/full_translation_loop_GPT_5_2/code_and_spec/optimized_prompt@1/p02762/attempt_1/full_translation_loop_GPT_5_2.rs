use std::io::{self, Read};

const N_MAX: usize = 100_000;

fn find_root(parent: &mut [i32], mut x: usize) -> usize {
    // C uses a fixed stack of size L_MAX+1 (21). In practice, union-by-size keeps
    // depth small, but to preserve semantics safely we use a Vec stack.
    let mut stack: Vec<usize> = Vec::new();
    while parent[x] >= 1 {
        stack.push(x);
        x = parent[x] as usize;
    }
    for v in stack {
        parent[v] = x as i32;
    }
    x
}

fn unite_set(parent: &mut [i32], x: usize, y: usize) {
    let mut rx = find_root(parent, x);
    let mut ry = find_root(parent, y);
    if rx == ry {
        return;
    }
    // parent[root] stores negative size; more negative => larger set.
    if parent[rx] < parent[ry] {
        parent[rx] += parent[ry];
        parent[ry] = rx as i32;
    } else {
        parent[ry] += parent[rx];
        parent[rx] = ry as i32;
    }
}

fn set_size(parent: &mut [i32], x: usize) -> i32 {
    let r = find_root(parent, x);
    -parent[r]
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let mut m: i32 = it.next().unwrap().parse().unwrap();
    let mut k: i32 = it.next().unwrap().parse().unwrap();

    // Arrays are 1-indexed, size n+1.
    // parent[i] = -size if i is root, else parent index (>=1).
    let mut parent = vec![0i32; n + 1];
    for i in 1..=n {
        parent[i] = -1;
    }

    // rmv initialized to 1 for each element (as in C).
    let mut rmv = vec![0i32; n + 1];
    for i in 1..=n {
        rmv[i] = 1;
    }

    while m > 0 {
        m -= 1;
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();

        let ra = find_root(&mut parent, a);
        let rb = find_root(&mut parent, b);
        if ra != rb {
            unite_set(&mut parent, a, b);
        }
        rmv[a] += 1;
        rmv[b] += 1;
    }

    while k > 0 {
        k -= 1;
        let c: usize = it.next().unwrap().parse().unwrap();
        let d: usize = it.next().unwrap().parse().unwrap();

        let rc = find_root(&mut parent, c);
        let rd = find_root(&mut parent, d);
        if rc == rd {
            rmv[c] += 1;
            rmv[d] += 1;
        }
    }

    // Output: set_size(i) - rmv[i], space-separated, newline at end.
    // Use i32 to match C int behavior.
    let mut out = String::new();
    for i in 1..=n {
        let val = set_size(&mut parent, i) - rmv[i];
        if i == n {
            out.push_str(&format!("{val}\n"));
        } else {
            out.push_str(&format!("{val} "));
        }
    }
    print!("{out}");
}
