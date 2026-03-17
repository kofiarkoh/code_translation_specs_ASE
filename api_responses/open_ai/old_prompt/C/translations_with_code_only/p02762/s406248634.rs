use std::io::{self, BufRead};

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
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let n = nums.next().unwrap();
    let m = nums.next().unwrap();
    let k = nums.next().unwrap();

    let mut s = vec![-1; N_MAX + 1];
    fill(&mut s, 1, n as usize, -1);
    let mut rmv = vec![1; N_MAX + 1];
    fill(&mut rmv, 1, n as usize, 1);

    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let a = nums.next().unwrap();
        let b = nums.next().unwrap();
        if find_root(&mut s, a as usize) != find_root(&mut s, b as usize) {
            unite_set(&mut s, a as usize, b as usize);
        }
        rmv[a as usize] += 1;
        rmv[b as usize] += 1;
    }

    for _ in 0..k {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let c = nums.next().unwrap();
        let d = nums.next().unwrap();
        if find_root(&mut s, c as usize) == find_root(&mut s, d as usize) {
            rmv[c as usize] += 1;
            rmv[d as usize] += 1;
        }
    }

    for i in 1..=n {
        print!("{}", set_size(&mut s, i as usize) - rmv[i as usize]);
        if i != n {
            print!(" ");
        } else {
            println!();
        }
    }
}
