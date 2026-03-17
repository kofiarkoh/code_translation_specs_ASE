use std::io::{self, Write};

const N_MAX: usize = 100000;
const L_MAX: usize = 20;

fn fill(a: &mut [i32], l: usize, r: usize, x: i32) {
    for i in l..=r {
        a[i] = x;
    }
}

fn find_root(a: &mut [i32], mut x: usize) -> usize {
    let mut s = [0; L_MAX + 1];
    let mut k = 0;

    while a[x] >= 1 {
        k += 1;
        s[k] = x;
        x = a[x] as usize;
    }

    while k > 0 {
        k -= 1;
        a[s[k]] = x as i32;
    }

    x
}

fn unite_set(a: &mut [i32], mut x: usize, mut y: usize) {
    x = find_root(a, x);
    y = find_root(a, y);
    if a[x] < a[y] {
        a[x] += a[y];
        a[y] = x as i32;
    } else {
        a[y] += a[x];
        a[x] = y as i32;
    }
}

fn set_size(a: &mut [i32], x: usize) -> i32 {
    -a[find_root(a, x)] // Return the size of the set
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    let mut s = vec![-1; N_MAX + 1];
    fill(&mut s, 1, n, -1);
    let mut rmv = vec![1; N_MAX + 1];
    fill(&mut rmv, 1, n, 1);

    for _ in 0..m {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        if find_root(&mut s, a) != find_root(&mut s, b) {
            unite_set(&mut s, a, b);
        }
        rmv[a] += 1;
        rmv[b] += 1;
    }

    for _ in 0..k {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let c: usize = iter.next().unwrap().parse().unwrap();
        let d: usize = iter.next().unwrap().parse().unwrap();
        if find_root(&mut s, c) == find_root(&mut s, d) {
            rmv[c] += 1;
            rmv[d] += 1;
        }
    }

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for i in 1..=n {
        let size = set_size(&mut s, i) - rmv[i];
        if i == n {
            writeln!(handle, "{}", size).unwrap();
        } else {
            write!(handle, "{} ", size).unwrap();
        }
    }
}
