use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Node {
    edge_to: [i32; 100],
    edge_cost: [i32; 100],
    edge_size: i32,
    cost: i32,
    visited: i32,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            edge_to: [0; 100],
            edge_cost: [0; 100],
            edge_size: 0,
            cost: 0,
            visited: 0,
        }
    }
}

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}

impl Scanner {
    fn new() -> Self {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        Self { buf: input, idx: 0 }
    }

    fn getchar_unlocked(&mut self) -> i32 {
        if self.idx >= self.buf.len() {
            // Emulate C getchar_unlocked() returning EOF (-1)
            -1
        } else {
            let c = self.buf[self.idx] as i32;
            self.idx += 1;
            c
        }
    }

    fn get_uint(&mut self) -> i32 {
        // Mirrors the C logic exactly, including returning the first non-digit
        // (or EOF) if the first char is not a digit.
        let mut n: i32 = 0;
        let mut c: i32 = self.getchar_unlocked();
        if c < 48 || 57 < c {
            return c;
        }
        while 47 < c && c < 58 {
            // Use wrapping ops to emulate C signed overflow behavior as closely as possible.
            n = n.wrapping_mul(10).wrapping_add(c & 0x0f);
            c = self.getchar_unlocked();
        }
        n
    }
}

fn put_uint(out: &mut Vec<u8>, mut n: i32) {
    if n == 0 {
        out.push(b'0');
        return;
    }
    // C uses char buf[11]; and prints digits for positive ints.
    // If n is negative (possible via overflow/UB in C), C's behavior is not well-defined;
    // we emulate the same digit-extraction loop with wrapping division/remainder semantics.
    let mut buf = [0u8; 11];
    let mut i: usize = 0;
    while n != 0 {
        let digit = (n % 10).wrapping_add(b'0' as i32) as u8;
        if i < buf.len() {
            buf[i] = digit;
        }
        i += 1;
        n /= 10;
        if i >= buf.len() {
            // Prevent out-of-bounds; C would overflow the buffer (UB).
            // We cap to preserve memory safety while keeping output as close as possible.
            break;
        }
    }
    while i > 0 {
        i -= 1;
        out.push(buf[i]);
    }
}

fn dijkstra_noheap(nodes: &mut [Node], n: usize) {
    for i in 0..n {
        nodes[i].cost = -1;
        nodes[i].visited = 0;
    }
    if n > 0 {
        nodes[0].cost = 0;
    }

    loop {
        let mut next_idx: Option<usize> = None;
        for i in 0..n {
            if nodes[i].visited != 0 || nodes[i].cost < 0 {
                continue;
            }
            match next_idx {
                None => next_idx = Some(i),
                Some(j) => {
                    if nodes[i].cost < nodes[j].cost {
                        next_idx = Some(i);
                    }
                }
            }
        }
        let Some(ni) = next_idx else { break };
        nodes[ni].visited = 1;

        // C copies the whole struct: Node u = *next;
        let u = nodes[ni];

        let esz = u.edge_size;
        // If edge_size is negative (possible due to bad input/UB), C loop won't run.
        if esz > 0 {
            for i in 0..(esz as usize) {
                let to = u.edge_to[i] as usize;
                let cost = u.cost.wrapping_add(u.edge_cost[i]);
                if to < n {
                    if nodes[to].cost < 0 || cost < nodes[to].cost {
                        nodes[to].cost = cost;
                    }
                } else {
                    // C would index out of bounds (UB). Ignore to preserve safety.
                }
            }
        }
    }
}

fn main() {
    let mut sc = Scanner::new();
    let n_i32 = sc.get_uint();
    let n = if n_i32 <= 0 { 0usize } else { n_i32 as usize };

    // C: Node node[n]; (VLA, uninitialized). We must initialize in Rust.
    // This may differ if C relied on uninitialized edge_size, but the code assigns
    // edge_size for each u it reads; we keep deterministic zero-init.
    let mut nodes = vec![Node::default(); n];

    for _ in 0..n {
        let u_i32 = sc.get_uint();
        let u = if u_i32 < 0 { 0usize } else { u_i32 as usize };
        let k = sc.get_uint();
        if u < n {
            nodes[u].edge_size = k;
            let kk = if k <= 0 { 0usize } else { k as usize };
            let limit = kk.min(100);
            for j in 0..limit {
                nodes[u].edge_to[j] = sc.get_uint();
                nodes[u].edge_cost[j] = sc.get_uint();
            }
            // If k > 100, C would write out of bounds (UB). We still must consume input.
            for _ in limit..kk {
                let _ = sc.get_uint();
                let _ = sc.get_uint();
            }
        } else {
            // C would write out of bounds (UB). We still must consume input.
            let kk = if k <= 0 { 0usize } else { k as usize };
            for _ in 0..kk {
                let _ = sc.get_uint();
                let _ = sc.get_uint();
            }
        }
    }

    dijkstra_noheap(&mut nodes, n);

    let mut out: Vec<u8> = Vec::new();
    for i in 0..n {
        put_uint(&mut out, i as i32);
        out.push(b' ');
        put_uint(&mut out, nodes[i].cost);
        out.push(b'\n');
    }

    print!("{}", String::from_utf8_lossy(&out));
}
