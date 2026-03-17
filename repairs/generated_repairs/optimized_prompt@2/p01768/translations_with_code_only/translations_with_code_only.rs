use std::io::{self, BufRead};

const INIT_CAP: usize = 10;

#[derive(Debug, Default)]
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
static mut ADJ: Option<[AdjList; 5000]> = None;
static mut ING: Option<[Rec; 5000]> = None;
static mut MIN: i32 = 100000;
static mut VISITED: [i32; 5000] = [0; 5000];

fn init(n: usize) {
    unsafe {
        ADJ = Some(Default::default());
        if let Some(adj) = &mut ADJ {
            for i in 0..n {
                adj[i] = AdjList {
                    to: Vec::with_capacity(INIT_CAP),
                    cap: INIT_CAP,
                    size: 0,
                };
            }
        }
    }
}

fn add(from: usize, to: usize) {
    unsafe {
        if let Some(adj) = &mut ADJ {
            if adj[from].size == adj[from].cap {
                adj[from].cap <<= 1; // Double the capacity
                adj[from].to.reserve(adj[from].cap - adj[from].to.len());
            }
            adj[from].to.push(to);
            adj[from].size += 1;
        }
    }
}

fn search_idx(str: &str) -> usize {
    unsafe {
        if let Some(ing) = &ING {
            for i in 0..N {
                if ing[i].name == str {
                    return i;
                }
            }
        }
    }
    N // Return N if not found, simulating the behavior of the original C code
}

fn dfs(i: usize) {
    unsafe {
        VISITED[i] = 1;
        MIN = MIN.min(ING.as_ref().unwrap()[i].cost);

        for &to in &ADJ.as_ref().unwrap()[i].to {
            if VISITED[to] == 0 {
                dfs(to);
            }
        }
    }
}

fn search_min_cost(i: usize) -> i32 {
    unsafe {
        MIN = 100000;
        VISITED = [0; 5000]; // Reset visited array

        dfs(i);

        ING.as_mut().unwrap()[i].cost = MIN; // Update cost
        MIN
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    unsafe {
        N = lines.next().unwrap().unwrap().parse().unwrap();
        ING = Some(Default::default());
        if let Some(ing) = &mut ING {
            for i in 0..N {
                let line = lines.next().unwrap().unwrap();
                let parts: Vec<&str> = line.split_whitespace().collect();
                ing[i] = Rec {
                    name: parts[0].to_string(),
                    cost: parts[1].parse().unwrap(),
                };
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
}
