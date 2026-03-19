use std::io::{self, Read};

const N_MAX: usize = 100000;
const L_MAX: usize = 20;

fn fill(a: &mut [i32], l: usize, r: usize, x: i32) {
    for i in l..=r {
        a[i] = x;
    }
}

fn find_root(a: &mut [i32], mut x: usize) -> usize {
    // Mirrors the C code's fixed-size stack (L_MAX+1).
    // Note: If the path length exceeds L_MAX, the C code would overflow this array (UB).
    // We emulate the same effective behavior by only recording up to L_MAX entries.
    let mut s = [0usize; L_MAX + 1];
    let mut k: usize = 0;

    while a[x] >= 1 {
        if k < L_MAX {
            k += 1;
            s[k] = x;
        }
        x = a[x] as usize;
    }
    while k > 0 {
        let idx = s[k];
        a[idx] = x as i32;
        k -= 1;
    }
    x
}

fn unite_set(a: &mut [i32], x: usize, y: usize) {
    let mut x = find_root(a, x);
    let mut y = find_root(a, y);
    if a[x] < a[y] {
        a[x] = a[x].wrapping_add(a[y]);
        a[y] = x as i32;
    } else {
        a[y] = a[y].wrapping_add(a[x]);
        a[x] = y as i32;
    }
}

fn set_size(a: &mut [i32], x: usize) -> i32 {
    let r = find_root(a, x);
    -a[r]
}

fn main() {
    // Read all input (scanf-like whitespace handling).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let mut m: i32 = it.next().unwrap().parse().unwrap();
    let mut k: i32 = it.next().unwrap().parse().unwrap();

    // Arrays sized as in C: N_MAX + 1, using 1..=n.
    let mut s = vec![0i32; N_MAX + 1];
    fill(&mut s, 1, n, -1);

    let mut rmv = vec![0i32; N_MAX + 1];
    fill(&mut rmv, 1, n, 1);

    while m > 0 {
        m -= 1;
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();

        let ra = find_root(&mut s, a);
        let rb = find_root(&mut s, b);
        if ra != rb {
            unite_set(&mut s, a, b);
        }
        rmv[a] = rmv[a].wrapping_add(1);
        rmv[b] = rmv[b].wrapping_add(1);
    }

    while k > 0 {
        k -= 1;
        let c: usize = it.next().unwrap().parse().unwrap();
        let d: usize = it.next().unwrap().parse().unwrap();

        let rc = find_root(&mut s, c);
        let rd = find_root(&mut s, d);
        if rc == rd {
            rmv[c] = rmv[c].wrapping_add(1);
            rmv[d] = rmv[d].wrapping_add(1);
        }
    }

    let mut out = String::new();
    for i in 1..=n {
        let val = set_size(&mut s, i).wrapping_sub(rmv[i]);
        if i == n {
            out.push_str(&format!("{val}\n"));
        } else {
            out.push_str(&format!("{val} "));
        }
    }
    print!("{out}");
}
