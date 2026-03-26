use std::io::{self, BufRead};
use std::collections::HashMap;

const INIT_CAP: usize = 10;

#[derive(Debug)]
struct AdjList {
    to: Vec<usize>,
    cap: usize,
    size: usize,
}

#[derive(Debug)]
struct Rec {
    name: String,
    cost: i32,
}

static mut N: usize = 0;
static mut ADJ: [AdjList; 5000] = Default::default();
static mut ING: [Rec; 5000] = Default::default();
static mut MIN: i32 = 0;
static mut VISITED: [bool; 5000] = Default::default();

fn init(n: usize) {
    unsafe {
        for i in 0..n {
            ADJ[i] = AdjList {
                to: Vec::with_capacity(INIT_CAP),
                cap: INIT_CAP,
                size: 0,
            };
        }
    }
}

fn add(from: usize, to: usize) {
    unsafe {
        if ADJ[from].size == ADJ[from].cap {
            ADJ[from].cap *= 2;
            ADJ[from].to.reserve(ADJ[from].cap - ADJ[from].to.len());
        }
        ADJ[from].to.push(to);
        ADJ[from].size += 1;
    }
}

fn search_idx(str: &str) -> usize {
    unsafe {
        for i in 0..N {
            if ING[i].name == str {
                return i;
            }
        }
    }
    N // Return N if not found, simulating the behavior of returning an out-of-bounds index
}

fn dfs(i: usize) {
    unsafe {
        VISITED[i] = true;
        MIN = MIN.min(ING[i].cost);

        for &to in &ADJ[i].to {
            if !VISITED[to] {
                dfs(to);
            }
        }
    }
}

fn search_min_cost(i: usize) -> i32 {
    unsafe {
        MIN = 100000;
        VISITED = [false; 5000];

        dfs(i);

        ING[i].cost = MIN;
        MIN
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    unsafe { N = n };

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let name = parts.next().unwrap().to_string();
        let cost: i32 = parts.next().unwrap().parse().unwrap();
        unsafe {
            ING[i] = Rec { name, cost };
        }
    }

    let m: usize = lines.next().unwrap().unwrap().parse().unwrap();
    init(n);

    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let s = parts.next().unwrap();
        let t = parts.next().unwrap();
        let idx_s = search_idx(s);
        let idx_t = search_idx(t);

        add(idx_s, idx_t);
        add(idx_t, idx_s);
    }

    let mut ans = 0;
    for i in 0..n {
        ans += search_min_cost(i);
    }

    println!("{}", ans);
}
