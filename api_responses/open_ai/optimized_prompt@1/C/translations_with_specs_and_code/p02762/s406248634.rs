use std::io::{self, BufRead};

const N_MAX: usize = 100_000;
const L_MAX: usize = 20;

fn fill(a: &mut [i32], l: usize, r: usize, x: i32) {
    for i in l..=r {
        a[i] = x;
    }
}

fn find_root(a: &mut [i32], x: usize) -> usize {
    let mut s = Vec::new();
    let mut current = x;

    while a[current] >= 1 {
        s.push(current);
        current = a[current] as usize;
    }

    for &node in s.iter().rev() {
        a[node] = current as i32;
    }

    current
}

fn unite_set(a: &mut [i32], x: usize, y: usize) {
    let root_x = find_root(a, x);
    let root_y = find_root(a, y);

    if a[root_x] < a[root_y] {
        a[root_x] += a[root_y];
        a[root_y] = root_x as i32;
    } else {
        a[root_y] += a[root_x];
        a[root_x] = root_y as i32;
    }
}

fn set_size(a: &[i32], x: usize) -> i32 {
    -a[find_root(&mut a.to_vec(), x)] // Create a mutable copy for find_root
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    let mut s = vec![-1; N_MAX + 1];
    fill(&mut s, 1, n, -1);
    let mut rmv = vec![1; N_MAX + 1];
    fill(&mut rmv, 1, n, 1);

    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let a: usize = parts.next().unwrap().parse().unwrap();
        let b: usize = parts.next().unwrap().parse().unwrap();

        if find_root(&mut s, a) != find_root(&mut s, b) {
            unite_set(&mut s, a, b);
        }
        rmv[a] += 1;
        rmv[b] += 1;
    }

    for _ in 0..k {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let c: usize = parts.next().unwrap().parse().unwrap();
        let d: usize = parts.next().unwrap().parse().unwrap();

        if find_root(&mut s, c) == find_root(&mut s, d) {
            rmv[c] += 1;
            rmv[d] += 1;
        }
    }

    for i in 1..=n {
        let size = set_size(&s, i) - rmv[i];
        print!("{}{}", size, if i == n { "\n" } else { " " });
    }
}
