use std::io::{self, BufRead};

const MAX: usize = 100_000;

fn root(x: usize, par: &mut [usize]) -> usize {
    if par[x] == x {
        return x;
    }
    par[x] = root(par[x], par); // Path compression
    par[x]
}

fn unite(x: usize, y: usize, par: &mut [usize]) -> bool {
    let rx = root(x, par);
    let ry = root(y, par);
    if rx == ry {
        return false;
    }
    par[rx] = ry;
    true
}

fn same(x: usize, y: usize, par: &[usize]) -> bool {
    root(x, par) == root(y, par)
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace();
    let n: usize = nums.next().unwrap().parse().unwrap();
    let m: usize = nums.next().unwrap().parse().unwrap();

    let mut a = vec![0; m];
    let mut b = vec![0; m];
    for i in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace();
        a[i] = nums.next().unwrap().parse::<usize>().unwrap() - 1;
        b[i] = nums.next().unwrap().parse::<usize>().unwrap() - 1;
    }

    let mut par = (0..n).collect::<Vec<usize>>(); // Initialize parent array
    let mut root_cnt = vec![1; n];

    let mut ans = vec![0; m];
    ans[m - 1] = (n * (n - 1)) / 2;
    root_cnt[root(b[m - 1], &mut par)] += root_cnt[root(a[m - 1], &mut par)];
    root_cnt[root(a[m - 1], &mut par)] = 0;
    unite(a[m - 1], b[m - 1], &mut par);

    if m >= 2 {
        ans[m - 2] = (n * (n - 1)) / 2 - 1;
    }

    for i in (0..m - 1).rev() {
        if same(a[i], b[i], &par) {
            ans[i - 1] = ans[i];
        } else {
            ans[i - 1] = ans[i] - root_cnt[root(b[i], &mut par)] * root_cnt[root(a[i], &mut par)];
            root_cnt[root(b[i], &mut par)] += root_cnt[root(a[i], &mut par)];
            root_cnt[root(a[i], &mut par)] = 0;
            unite(a[i], b[i], &mut par);
        }
    }

    for i in 0..m {
        println!("{}", ans[i]);
    }
}
