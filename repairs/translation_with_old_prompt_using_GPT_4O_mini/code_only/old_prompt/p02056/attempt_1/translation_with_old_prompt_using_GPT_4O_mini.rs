use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::cmp::Ordering;
use std::ptr::{self, NonNull};
use std::slice;

type i32 = i32;
type i64 = i64;

struct BinaryHeap {
    array: NonNull<u8>,
    heap_size: usize,
    max_size: usize,
    val_size: usize,
    cmp: fn(&[u8], &[u8]) -> Ordering,
}

impl BinaryHeap {
    fn new(val_size: usize, cmp_func: fn(&[u8], &[u8]) -> Ordering) -> Self {
        let layout = Layout::from_size_align(std::mem::size_of::<BinaryHeap>(), std::mem::align_of::<BinaryHeap>()).unwrap();
        let ptr = unsafe { alloc_zeroed(layout) as *mut BinaryHeap };
        let array = unsafe { alloc_zeroed(Layout::from_size_align(val_size * 2, 1).unwrap()) };
        let heap = unsafe { &mut *ptr };
        heap.array = NonNull::new(array).unwrap();
        heap.heap_size = 0;
        heap.max_size = 1;
        heap.val_size = val_size;
        heap.cmp = cmp_func;
        heap.clone()
    }

    fn is_empty(&self) -> bool {
        self.heap_size == 0
    }

    fn push(&mut self, val: &[u8]) {
        if self.heap_size == self.max_size {
            self.max_size = 2 * self.max_size + 1;
            let new_layout = Layout::from_size_align(self.val_size * (self.max_size + 1), 1).unwrap();
            let new_array = unsafe { realloc(self.array.as_ptr(), new_layout.size(), new_layout.align()) };
            self.array = NonNull::new(new_array).unwrap();
        }
        self.heap_size += 1;
        let array = unsafe { slice::from_raw_parts_mut(self.array.as_ptr(), self.val_size * (self.max_size + 1)) };
        let k = self.heap_size;
        let val_size = self.val_size;
        let cmp = self.cmp;
        array[k * val_size..(k + 1) * val_size].copy_from_slice(val);
        let mut k = k;
        while k > 1 {
            let parent = k / 2;
            if cmp(&array[parent * val_size..(parent + 1) * val_size], &array[k * val_size..(k + 1) * val_size]) != Ordering::Greater {
                return;
            }
            array.swap(parent * val_size..(parent + 1) * val_size, k * val_size..(k + 1) * val_size);
            k = parent;
        }
    }

    fn pop(&mut self, res: &mut [u8]) {
        let array = unsafe { slice::from_raw_parts_mut(self.array.as_ptr(), self.val_size * (self.max_size + 1)) };
        let val_size = self.val_size;
        if !res.is_empty() {
            res.copy_from_slice(&array[val_size..(val_size + res.len())]);
        }
        array.copy_within(val_size..(val_size * self.heap_size), val_size);
        self.heap_size -= 1;
        let cmp = self.cmp;
        let n = self.heap_size;
        let mut k = 1;
        while 2 * k + 1 <= n {
            let c = cmp(&array[2 * k * val_size..(2 * k + 1) * val_size], &array[(2 * k + 1) * val_size..(2 * k + 2) * val_size]);
            let next = 2 * k + if c == Ordering::Less { 0 } else { 1 };
            if cmp(&array[k * val_size..(k + 1) * val_size], &array[next * val_size..(next + 1) * val_size]) != Ordering::Greater {
                return;
            }
            array.swap(k * val_size..(k + 1) * val_size, next * val_size..(next + 1) * val_size);
            k = next;
        }
        if 2 * k <= n && cmp(&array[k * val_size..(k + 1) * val_size], &array[2 * k * val_size..(2 * k + 1) * val_size]) == Ordering::Greater {
            array.swap(k * val_size..(k + 1) * val_size, 2 * k * val_size..(2 * k + 1) * val_size);
        }
    }
}

struct GraphEdge {
    vertex: i32,
    next: i32,
    cost: i32,
}

struct Graph {
    edge: NonNull<GraphEdge>,
    start: NonNull<i32>,
    pointer: i32,
    vertex_num: i32,
    edge_max_size: i32,
}

impl Graph {
    fn new(vertex_num: i32) -> Self {
        let edge = unsafe { alloc_zeroed(Layout::from_size_align(std::mem::size_of::<GraphEdge>(), 1).unwrap()) };
        let start = unsafe { alloc_zeroed(Layout::from_size_align((vertex_num * std::mem::size_of::<i32>()) as usize, 1).unwrap()) };
        let g = Graph {
            edge: NonNull::new(edge).unwrap(),
            start: NonNull::new(start).unwrap(),
            pointer: 0,
            vertex_num,
            edge_max_size: 1,
        };
        for i in 0..vertex_num {
            unsafe { *start.as_ptr().add(i as usize) = -1 };
        }
        g
    }

    fn add_edge(&mut self, from: i32, to: i32, cost: i32) {
        if self.pointer == self.edge_max_size {
            self.edge_max_size *= 2;
            let new_edge = unsafe { realloc(self.edge.as_ptr() as *mut u8, (self.edge_max_size * std::mem::size_of::<GraphEdge>()) as usize) };
            self.edge = NonNull::new(new_edge as *mut GraphEdge).unwrap();
        }
        unsafe {
            let edge_ptr = self.edge.as_ptr().add(self.pointer as usize);
            (*edge_ptr).vertex = to;
            (*edge_ptr).next = *self.start.as_ptr().add(from as usize);
            (*edge_ptr).cost = cost;
            *self.start.as_ptr().add(from as usize) = self.pointer;
        }
        self.pointer += 1;
    }
}

struct Node {
    v: i32,
    d: i64,
}

fn cmp_node(a: &[u8], b: &[u8]) -> Ordering {
    let a: &Node = unsafe { &*(a.as_ptr() as *const Node) };
    let b: &Node = unsafe { &*(b.as_ptr() as *const Node) };
    a.d.cmp(&b.d)
}

fn run() {
    let mut n: i32 = 0;
    let mut m: i32 = 0;
    let mut k: i32 = 0;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
    n = nums[0];
    m = nums[1];
    k = nums[2];

    let p = vec![0; (3 * n) as usize];
    let c = &p[n as usize..];
    let t = &p[(2 * n) as usize..];

    for i in 0..(3 * n) {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        p[i as usize] = input.trim().parse().unwrap();
    }

    let sup = 1_000_000_001;
    let mut g = Graph::new(2 * n);
    for i in 0..n {
        g.add_edge(i, i + n, sup - p[i as usize]);
    }
    for _ in 0..m {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let nums: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let (mut a, mut b, d) = (nums[0] - 1, nums[1] - 1, nums[2]);
        g.add_edge(a, b, d);
        g.add_edge(b, a, d);
        a += n;
        b += n;
        g.add_edge(a, b, d);
        g.add_edge(b, a, d);
    }

    let mut dp = vec![1_000_000_002; (2 * n) as usize];
    let mut h = BinaryHeap::new(std::mem::size_of::<Node>(), cmp_node);
    h.push(&Node { v: 0, d: 0 }.to_ne_bytes());

    while !h.is_empty() {
        let mut t = Node { v: 0, d: 0 };
        h.pop(&mut t.to_ne_bytes());
        if t.d > dp[t.v as usize] {
            continue;
        }
        let start = unsafe { g.start.as_ptr().add(t.v as usize) };
        let mut p = unsafe { *start };
        while p != -1 {
            let u = unsafe { *g.edge.as_ptr().add(p as usize) }.vertex;
            let d = t.d + unsafe { *g.edge.as_ptr().add(p as usize) }.cost as i64;
            if d >= dp[u as usize] {
                continue;
            }
            dp[u as usize] = d;
            h.push(&Node { v: u, d }.to_ne_bytes());
            p = unsafe { *g.edge.as_ptr().add(p as usize) }.next;
        }
    }

    let mut max = vec![1_000_000_002; (k + 1) as usize];
    for i in 0..n {
        let v = (sup - dp[(i + n) as usize]) - dp[i as usize] + t[i as usize];
        max[c[i as usize] as usize] = max[c[i as usize] as usize].max(v);
    }
    for i in 1..=k {
        println!("{}", max[i as usize]);
    }
}

fn main() {
    run();
}
