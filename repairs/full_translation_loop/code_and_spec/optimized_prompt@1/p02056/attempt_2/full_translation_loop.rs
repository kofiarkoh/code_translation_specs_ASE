use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Node {
    v: i32,
    d: i64,
}

struct BinaryHeap {
    array: Vec<Node>,
    cmp: fn(&Node, &Node) -> Ordering,
}

impl BinaryHeap {
    fn new(cmp: fn(&Node, &Node) -> Ordering) -> Self {
        Self {
            array: Vec::new(),
            cmp,
        }
    }

    fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    fn push(&mut self, val: Node) {
        self.array.push(val);
        self.bubble_up(self.array.len() - 1);
    }

    fn pop(&mut self) -> Option<Node> {
        if self.is_empty() {
            return None;
        }
        let root = self.array[0].clone(); // Clone the root node
        let last = self.array.pop().unwrap();
        if !self.is_empty() {
            self.array[0] = last;
            self.bubble_down(0);
        }
        Some(root)
    }

    fn bubble_up(&mut self, index: usize) {
        let mut child = index;
        while child > 0 {
            let parent = (child - 1) / 2;
            if (self.cmp)(&self.array[child], &self.array[parent]) != Ordering::Less {
                break;
            }
            self.array.swap(child, parent);
            child = parent;
        }
    }

    fn bubble_down(&mut self, index: usize) {
        let mut parent = index;
        let len = self.array.len();
        loop {
            let left = 2 * parent + 1;
            let right = 2 * parent + 2;
            let mut smallest = parent;

            if left < len && (self.cmp)(&self.array[left], &self.array[smallest]) == Ordering::Less {
                smallest = left;
            }
            if right < len && (self.cmp)(&self.array[right], &self.array[smallest]) == Ordering::Less {
                smallest = right;
            }
            if smallest == parent {
                break;
            }
            self.array.swap(parent, smallest);
            parent = smallest;
        }
    }
}

struct Graph {
    edges: Vec<(i32, i32, i32)>, // (to, next, cost)
    start: Vec<i32>,
    pointer: usize,
    vertex_num: i32,
    edge_max_size: usize,
}

impl Graph {
    fn new(vertex_num: i32) -> Self {
        Self {
            edges: Vec::new(),
            start: vec![-1; vertex_num as usize],
            pointer: 0,
            vertex_num,
            edge_max_size: 1,
        }
    }

    fn add_edge(&mut self, from: i32, to: i32, cost: i32) {
        if self.pointer == self.edge_max_size {
            self.edge_max_size *= 2;
            self.edges.resize(self.edge_max_size * 2, (0, 0, 0));
        }
        self.edges.push((to, self.start[from as usize], cost));
        self.start[from as usize] = self.pointer as i32;
        self.pointer += 1;
    }
}

fn cmp_node(a: &Node, b: &Node) -> Ordering {
    a.d.cmp(&b.d)
}

fn run() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace();
    let n: i32 = params.next().unwrap().parse().unwrap();
    let m: i32 = params.next().unwrap().parse().unwrap();
    let k: i32 = params.next().unwrap().parse().unwrap();

    let mut p: Vec<i32> = Vec::with_capacity(3 * n as usize);
    for _ in 0..3 * n {
        let value: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        p.push(value);
    }

    let sup = 1_000_000_001;
    let mut g = Graph::new(2 * n);
    for i in 0..n {
        g.add_edge(i, i + n, sup - p[i as usize]);
    }

    for _ in 0..m {
        let edge_line = lines.next().unwrap().unwrap();
        let mut edge_params = edge_line.split_whitespace();
        let a: i32 = edge_params.next().unwrap().parse().unwrap() - 1;
        let b: i32 = edge_params.next().unwrap().parse().unwrap() - 1;
        let d: i32 = edge_params.next().unwrap().parse().unwrap();

        g.add_edge(a, b, d);
        g.add_edge(b, a, d);
        g.add_edge(a + n, b + n, d);
        g.add_edge(b + n, a + n, d);
    }

    let mut dp = vec![1_000_000_001 * 2 * n as i64; (2 * n) as usize];
    dp[0] = 0;

    let mut heap = BinaryHeap::new(cmp_node);
    heap.push(Node { v: 0, d: 0 });

    while !heap.is_empty() {
        let t = heap.pop().unwrap();
        if t.d > dp[t.v as usize] {
            continue;
        }
        let mut p = g.start[t.v as usize];
        while p != -1 {
            let (u, _, cost) = g.edges[p as usize];
            let d = t.d + cost as i64;
            if d >= dp[u as usize] {
                p = g.edges[p as usize].1; // next
                continue;
            }
            dp[u as usize] = d;
            heap.push(Node { v: u, d });
            p = g.edges[p as usize].1; // next
        }
    }

    let mut max = vec![2 - (1_000_000_001 * 2 * n) as i64; (k + 1) as usize];
    for i in 0..n {
        let v = (sup as i64 - dp[(i + n) as usize]) - dp[i as usize] as i64 + p[i as usize] as i64;
        max[p[i as usize] as usize] = max[p[i as usize] as usize].max(v);
    }

    for i in 1..=k {
        println!("{}", max[i as usize]);
    }
}

fn main() {
    run();
}
