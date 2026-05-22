use std::cmp::Ordering;
use std::io::{self, Read};

type I32 = i32;
type I64 = i64;

#[derive(Clone, Copy)]
struct Edge {
    vertex: I32,
    next: I32,
    cost: I32, // edge_weight
}

struct Graph {
    edge: Vec<Edge>,
    start: Vec<I32>,
    pointer: I32,
}

fn new_graph(vertex_num: usize) -> Graph {
    let mut start = vec![-1i32; vertex_num];
    // C allocates 1 edge initially (calloc(1, sizeof(graph_edge)))
    let edge = vec![Edge {
        vertex: 0,
        next: 0,
        cost: 0,
    }];
    Graph {
        edge,
        start,
        pointer: 0,
    }
}

fn add_edge(g: &mut Graph, from: I32, to: I32, cost: I32) {
    let from_usize = from as usize;
    let ptr = g.pointer as usize;

    // C grows when pointer == edge_max_size, doubling capacity.
    // In Rust Vec grows automatically; we just ensure index exists.
    if ptr >= g.edge.len() {
        // mimic realloc growth pattern (at least double)
        let new_len = std::cmp::max(1, g.edge.len() * 2);
        g.edge.resize(
            new_len,
            Edge {
                vertex: 0,
                next: 0,
                cost: 0,
            },
        );
    }

    let next = g.start[from_usize];
    g.edge[ptr] = Edge {
        vertex: to,
        next,
        cost,
    };
    g.start[from_usize] = g.pointer;
    g.pointer += 1;
}

#[derive(Clone, Copy)]
struct Node {
    v: I32,
    d: I64,
}

// C comparator:
// i64 d = a.d - b.d; return sign(d)
fn cmp_node(a: &Node, b: &Node) -> Ordering {
    let diff = a.d.wrapping_sub(b.d); // preserve potential overflow behavior
    if diff == 0 {
        Ordering::Equal
    } else if diff < 0 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

// A simple binary min-heap with 1-based indexing to match C behavior closely.
struct BinaryHeapMin {
    a: Vec<Node>, // a[0] unused
    size: usize,
}

impl BinaryHeapMin {
    fn new() -> Self {
        Self {
            a: vec![Node { v: 0, d: 0 }],
            size: 0,
        }
    }
    fn is_empty(&self) -> bool {
        self.size == 0
    }
    fn push(&mut self, val: Node) {
        self.size += 1;
        if self.size >= self.a.len() {
            self.a.push(val);
        } else {
            self.a[self.size] = val;
        }
        let mut k = self.size;
        while k > 1 {
            let parent = k / 2;
            if cmp_node(&self.a[parent], &self.a[k]) != Ordering::Greater {
                return;
            }
            self.a.swap(parent, k);
            k = parent;
        }
    }
    fn pop(&mut self) -> Node {
        // Caller ensures non-empty (as in C code).
        let res = self.a[1];
        self.a[1] = self.a[self.size];
        self.size -= 1;

        let n = self.size;
        let mut k = 1usize;
        while 2 * k + 1 <= n {
            let left = 2 * k;
            let right = 2 * k + 1;
            let c = cmp_node(&self.a[left], &self.a[right]);
            let next = if c != Ordering::Greater { left } else { right };
            if cmp_node(&self.a[k], &self.a[next]) != Ordering::Greater {
                return res;
            }
            self.a.swap(k, next);
            k = next;
        }
        if 2 * k <= n && cmp_node(&self.a[k], &self.a[2 * k]) == Ordering::Greater {
            self.a.swap(k, 2 * k);
        }
        res
    }
}

struct Scanner {
    input: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).unwrap();
        Self {
            input: s.into_bytes(),
            idx: 0,
        }
    }
    fn next_i32(&mut self) -> i32 {
        while self.idx < self.input.len() && self.input[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        let mut sign = 1i32;
        if self.idx < self.input.len() && self.input[self.idx] == b'-' {
            sign = -1;
            self.idx += 1;
        }
        let mut x: i32 = 0;
        while self.idx < self.input.len() && !self.input[self.idx].is_ascii_whitespace() {
            x = x.wrapping_mul(10).wrapping_add((self.input[self.idx] - b'0') as i32);
            self.idx += 1;
        }
        x.wrapping_mul(sign)
    }
}

fn run() {
    let mut sc = Scanner::new();
    let n: I32 = sc.next_i32();
    let mut m: I32 = sc.next_i32();
    let k: I32 = sc.next_i32();

    // C: i32 *p = ALLOC(3*n, i32); c = p+n; t = c+n;
    let nn = n as usize;
    let mut arr = vec![0i32; 3 * nn];
    for i in 0..(3 * nn) {
        arr[i] = sc.next_i32();
    }
    let p = &arr[0..nn];
    let c = &arr[nn..2 * nn];
    let t = &arr[2 * nn..3 * nn];

    let sup: I32 = 1_000_000_000 + 1;

    let mut g = new_graph((2 * n) as usize);
    for i in 0..nn {
        add_edge(&mut g, i as I32, (i as I32) + n, sup.wrapping_sub(p[i]));
    }

    while m > 0 {
        let mut a: I32 = sc.next_i32();
        let mut b: I32 = sc.next_i32();
        let d: I32 = sc.next_i32();
        a -= 1;
        b -= 1;

        add_edge(&mut g, a, b, d);
        add_edge(&mut g, b, a, d);

        a = a.wrapping_add(n);
        b = b.wrapping_add(n);
        add_edge(&mut g, a, b, d);
        add_edge(&mut g, b, a, d);

        m -= 1;
    }

    // dp init: dp[0]=0 (calloc), dp[i]=1000000001*2*n for i>=1
    let mut dp = vec![0i64; (2 * n) as usize];
    let inf: I64 = (1_000_000_001i64)
        .wrapping_mul(2)
        .wrapping_mul(n as i64);
    for i in 1..dp.len() {
        dp[i] = inf;
    }

    let mut h = BinaryHeapMin::new();
    h.push(Node { v: 0, d: 0 });

    while !h.is_empty() {
        let cur = h.pop();
        let v_usize = cur.v as usize;
        if cur.d > dp[v_usize] {
            continue;
        }
        let mut eidx = g.start[v_usize];
        while eidx != -1 {
            let e = g.edge[eidx as usize];
            let u = e.vertex as usize;
            let nd = cur.d.wrapping_add(e.cost as i64);
            if nd >= dp[u] {
                eidx = e.next;
                continue;
            }
            dp[u] = nd;
            h.push(Node {
                v: e.vertex,
                d: nd,
            });
            eidx = e.next;
        }
    }

    // max array: size k+1, max[i]=2 - 1000000001*2*n for i>=1, max[0]=0 (calloc)
    let mut maxv = vec![0i64; (k as usize) + 1];
    let init_max: I64 = 2i64.wrapping_sub(inf);
    for i in 1..=k as usize {
        maxv[i] = init_max;
    }

    for i in 0..nn {
        let v = (sup as i64)
            .wrapping_sub(dp[i + nn])
            .wrapping_sub(dp[i])
            .wrapping_add(t[i] as i64);
        let ci = c[i] as usize;
        if v > maxv[ci] {
            maxv[ci] = v;
        }
    }

    let mut out = String::new();
    for i in 1..=k as usize {
        out.push_str(&format!("{}\n", maxv[i]));
    }
    print!("{out}");
}

fn main() {
    run();
}
