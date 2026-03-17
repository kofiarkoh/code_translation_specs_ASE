use std::io::{self, BufRead};
use std::ptr;
use std::mem;

const INIT_CAP: usize = 10;
const MAX_N: usize = 5000;

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
static mut ADJ: [AdjList; MAX_N] = [AdjList { to: Vec::new(), cap: 0, size: 0 }; MAX_N];
static mut ING: [Rec; MAX_N] = [Rec { name: String::new(), cost: 0 }; MAX_N];
static mut MIN: i32 = 100000;
static mut VISITED: [i32; MAX_N] = [0; MAX_N];

fn init(n: usize) {
    unsafe {
        for i in 0..n {
            ADJ[i].to = Vec::with_capacity(INIT_CAP);
            ADJ[i].cap = INIT_CAP;
            ADJ[i].size = 0;
        }
    }
}

fn add(from: usize, to: usize) {
    unsafe {
        if ADJ[from].size == ADJ[from].cap {
            ADJ[from].cap <<= 1; // Double the capacity
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
    N // Return N if not found, simulating the behavior of the original C code
}

fn dfs(i: usize) {
    unsafe {
        VISITED[i] = 1;
        MIN = MIN.min(ING[i].cost);

        for &to in &ADJ[i].to {
            if VISITED[to] == 0 {
                dfs(to);
            }
        }
    }
}

fn search_min_cost(i: usize) -> i32 {
    unsafe {
        MIN = 100000;
        VISITED = [0; MAX_N]; // Reset visited array

        dfs(i);

        ING[i].cost = MIN; // Update cost
        MIN
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    unsafe {
        N = lines.next().unwrap().unwrap().parse().unwrap();
        for i in 0..N {
            let line = lines.next().unwrap().unwrap();
            let parts: Vec<&str> = line.split_whitespace().collect();
            ING[i].name = parts[0].to_string();
            ING[i].cost = parts[1].parse().unwrap();
        }

        let m: usize = lines.next().unwrap().unwrap().parse().unwrap();
        init(N);
        for _ in 0..m {
            let line = lines.next().unwrap().unwrap();
            let parts: Vec<&str> = line.split_whitespace().collect();
            let idx_s = search_idx(parts[0]);
            let idx_t = search_idx(parts[1]);

            add(idx_s, idx_t);
            add(idx_t, idx_s);
        }

        let mut ans = 0;
        for i in 0..N {
            ans += search_min_cost(i);
        }

        println!("{}", ans);
    }
}
