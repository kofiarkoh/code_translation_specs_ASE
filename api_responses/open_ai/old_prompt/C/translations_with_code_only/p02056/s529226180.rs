use std::alloc::{alloc_zeroed, realloc, Layout};
use std::cmp::Ordering;
use std::io::{self, Read};
use std::ptr;

type i32 = i32;
type i64 = i64;

struct BinaryHeap {
    array: *mut u8,
    heap_size: usize,
    max_size: usize,
    val_size: usize,
    cmp: fn(*const u8, *const u8) -> i32,
}

impl BinaryHeap {
    fn new(val_size: usize, cmp_func: fn(*const u8, *const u8) -> i32) -> *mut Self {
        let layout = Layout::new::<Self>();
        let h = unsafe { alloc_zeroed(layout) as *mut Self };
        unsafe {
            (*h).array = alloc_zeroed(Layout::from_size_align(val_size * (1 + 1), 1).unwrap());
            (*h).heap_size = 0;
            (*h).max_size = 1;
            (*h).val_size = val_size;
            (*h).cmp = cmp_func;
        }
        h
    }

    fn is_empty(&self) -> bool {
        self.heap_size == 0
    }

    fn swap(&self, a: *mut u8, b: *mut u8) {
        let val_size = self.val_size;
        if val_size % 8 == 0 {
            let a = a as *mut u64;
            let b = b as *mut u64;
            let count = val_size / 8;
            for i in 0..count {
                unsafe {
                    let tmp = *a;
                    *a = *b;
                    *b = tmp;
                    a = a.add(1);
                    b = b.add(1);
                }
            }
        } else {
            let a = a as *mut u8;
            let b = b as *mut u8;
            for i in 0..val_size {
                unsafe {
                    let tmp = *a;
                    *a = *b;
                    *b = tmp;
                    a = a.add(1);
                    b = b.add(1);
                }
            }
        }
    }

    fn copy(&self, dst: *mut u8, src: *const u8) {
        let val_size = self.val_size;
        if val_size % 8 == 0 {
            let src = src as *const u64;
            let dst = dst as *mut u64;
            let count = val_size / 8;
            for _ in 0..count {
                unsafe {
                    *dst = *src;
                    dst = dst.add(1);
                    src = src.add(1);
                }
            }
        } else {
            let src = src as *const u8;
            let dst = dst as *mut u8;
            for _ in 0..val_size {
                unsafe {
                    *dst = *src;
                    dst = dst.add(1);
                    src = src.add(1);
                }
            }
        }
    }

    fn push(&mut self, val: *const u8) {
        if self.heap_size == self.max_size {
            self.max_size = 2 * self.max_size + 1;
            let new_layout = Layout::from_size_align(self.val_size * (self.max_size + 1), 1).unwrap();
            self.array = unsafe { realloc(self.array, new_layout.size(), new_layout.size()) };
        }
        self.heap_size += 1;
        let array = self.array;
        let k = self.heap_size;
        let val_size = self.val_size;
        let cmp = self.cmp;
        unsafe {
            self.copy(array.add(k * val_size), val);
            let mut k = k;
            while k > 1 {
                let parent = k / 2;
                if cmp(array.add(parent * val_size), array.add(k * val_size)) <= 0 {
                    return;
                }
                self.swap(array.add(parent * val_size), array.add(k * val_size));
                k = parent;
            }
        }
    }

    fn pop(&mut self, res: *mut u8) {
        let array = self.array;
        let val_size = self.val_size;
        unsafe {
            if !res.is_null() {
                self.copy(res, array.add(val_size));
            }
            self.copy(array.add(val_size), array.add(val_size * self.heap_size));
            self.heap_size -= 1;
            let cmp = self.cmp;
            let n = self.heap_size;
            let mut k = 1;
            while 2 * k + 1 <= n {
                let c = cmp(array.add(val_size * 2 * k), array.add(val_size * (2 * k + 1)));
                let next = 2 * k + if c <= 0 { 0 } else { 1 };
                if cmp(array.add(val_size * k), array.add(val_size * next)) <= 0 {
                    return;
                }
                self.swap(array.add(val_size * k), array.add(val_size * next));
                k = next;
            }
            if 2 * k <= n && cmp(array.add(val_size * k), array.add(val_size * 2 * k)) > 0 {
                self.swap(array.add(val_size * k), array.add(val_size * 2 * k));
            }
        }
    }
}

struct GraphEdge {
    vertex: i32,
    next: i32,
    cost: i32,
}

struct Graph {
    edge: *mut GraphEdge,
    start: *mut i32,
    pointer: i32,
    vertex_num: i32,
    edge_max_size: i32,
}

impl Graph {
    fn new(vertex_num: i32) -> *mut Self {
        let layout = Layout::new::<Self>();
        let g = unsafe { alloc_zeroed(layout) as *mut Self };
        unsafe {
            (*g).edge = alloc_zeroed(Layout::new::<GraphEdge>());
            (*g).start = alloc_zeroed(Layout::array::<i32>(vertex_num as usize).unwrap());
            (*g).pointer = 0;
            (*g).vertex_num = vertex_num;
            (*g).edge_max_size = 1;
            for i in 0..vertex_num {
                *(*g).start.add(i as usize) = -1;
            }
        }
        g
    }

    fn add_edge(&mut self, from: i32, to: i32, cost: i32) {
        if self.pointer == self.edge_max_size {
            self.edge_max_size *= 2;
            let new_layout = Layout::array::<GraphEdge>(self.edge_max_size as usize).unwrap();
            self.edge = unsafe { realloc(self.edge as *mut u8, new_layout.size()) as *mut GraphEdge };
        }
        unsafe {
            *self.edge.add(self.pointer as usize) = GraphEdge {
                vertex: to,
                next: *self.start.add(from as usize),
                cost,
            };
            *self.start.add(from as usize) = self.pointer;
            self.pointer += 1;
        }
    }
}

#[derive(Copy, Clone)]
struct Node {
    v: i32,
    d: i64,
}

fn cmp_node(a: *const u8, b: *const u8) -> i32 {
    let a = unsafe { &*(a as *const Node) };
    let b = unsafe { &*(b as *const Node) };
    a.d.cmp(&b.d) as i32
}

fn run() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    
    let n: i32 = iter.next().unwrap().parse().unwrap();
    let m: i32 = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    
    let mut p = vec![0; (3 * n) as usize];
    for i in 0..(3 * n) {
        p[i as usize] = iter.next().unwrap().parse().unwrap();
    }
    
    let sup = 1_000_000_001;
    let g = unsafe { Graph::new(2 * n) };
    for i in 0..n {
        unsafe {
            (*g).add_edge(i, i + n, sup - p[i as usize]);
        }
    }
    
    for _ in 0..m {
        let a: i32 = iter.next().unwrap().parse().unwrap();
        let b: i32 = iter.next().unwrap().parse().unwrap();
        let d: i32 = iter.next().unwrap().parse().unwrap();
        let a = a - 1;
        let b = b - 1;
        unsafe {
            (*g).add_edge(a, b, d);
            (*g).add_edge(b, a, d);
            (*g).add_edge(a + n, b + n, d);
            (*g).add_edge(b + n, a + n, d);
        }
    }
    
    let mut dp = vec![1_000_000_001 * 2 * n as i64; (2 * n) as usize];
    let h = unsafe { BinaryHeap::new(std::mem::size_of::<Node>(), cmp_node) };
    unsafe {
        (*h).push(&Node { v: 0, d: 0 } as *const _ as *const u8);
    }
    
    while !unsafe { (*h).is_empty() } {
        let mut t = Node { v: 0, d: 0 };
        unsafe { (*h).pop(&mut t as *mut _ as *mut u8) };
        if t.d > dp[t.v as usize] {
            continue;
        }
        for mut p in unsafe { (*g).start.add(t.v as usize) } {
            while p != -1 {
                let u = unsafe { (*(*g).edge.add(p as usize)).vertex };
                let d = t.d + unsafe { (*(*g).edge.add(p as usize)).cost as i64 };
                if d >= dp[u as usize] {
                    break;
                }
                dp[u as usize] = d;
                unsafe { (*h).push(&Node { v: u, d }) };
                p = unsafe { (*(*g).edge.add(p as usize)).next };
            }
        }
    }
    
    let mut max = vec![2 - (1_000_000_001 * 2 * n) as i64; (k + 1) as usize];
    for i in 0..n {
        let v = (sup - dp[(i + n) as usize]) - dp[i as usize] + p[i as usize];
        max[p[i as usize] as usize] = max[p[i as usize] as usize].max(v);
    }
    
    for i in 1..=k {
        println!("{}", max[i as usize]);
    }
}

fn main() {
    run();
}
