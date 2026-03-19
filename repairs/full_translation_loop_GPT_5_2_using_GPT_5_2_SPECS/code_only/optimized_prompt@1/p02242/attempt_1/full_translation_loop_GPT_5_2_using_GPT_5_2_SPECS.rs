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

    // Mirrors the C get_uint() exactly:
    // - reads one char
    // - if not '0'..'9', returns that char code (or -1 for EOF)
    // - otherwise parses consecutive digits into an int (with wrapping like C int)
    fn get_uint(&mut self) -> i32 {
        let mut n: i32 = 0;
        let mut c: i32 = self.getchar_unlocked();
        if c < 48 || 57 < c {
            return c;
        }
        while 47 < c && c < 58 {
            // (c & 0xf) is used in C; keep it.
            n = n.wrapping_mul(10).wrapping_add(c & 0xf);
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
    // C uses char buf[11]; and prints digits of n%10 while n != 0.
    // For negative n, C's % and / keep n moving toward 0; it will print
    // non-digit bytes (e.g., '/' for -1). We preserve that behavior.
    let mut buf = [0u8; 11];
    let mut i: usize = 0;
    while n != 0 {
        let digit = (n % 10).wrapping_add(b'0' as i32) as u8;
        if i < buf.len() {
            buf[i] = digit;
        }
        i += 1;
        n /= 10;
    }
    while i != 0 {
        i -= 1;
        // If i exceeded 11 due to unexpected behavior, C would overflow the buffer (UB).
        // Here we clamp to preserve output as best as possible without UB.
        let ch = if i < buf.len() { buf[i] } else { b'?' };
        out.push(ch);
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
        for i in 0..(esz as usize) {
            let to = u.edge_to[i] as usize;
            let cost = u.cost.wrapping_add(u.edge_cost[i]);
            if to < n {
                if nodes[to].cost < 0 || cost < nodes[to].cost {
                    nodes[to].cost = cost;
                }
            } else {
                // In C, out-of-bounds would be UB; we avoid crashing.
                // No external spec is given, so best-effort: ignore.
            }
        }
    }
}

fn main() {
    let mut sc = Scanner::new();
    let n_i32 = sc.get_uint();
    let n = if n_i32 < 0 { 0usize } else { n_i32 as usize };

    // C uses VLA Node node[n]; uninitialized. We must initialize in Rust.
    // This may differ if C relied on uninitialized fields, but the code assigns
    // edge_size for each u read; other nodes would be UB in C if accessed.
    let mut nodes = vec![Node::default(); n];

    for _ in 0..n {
        let u = sc.get_uint() as usize;
        if u >= n {
            // Would be UB in C; ignore to avoid panic.
            // Still consume input similarly.
            let k = sc.get_uint();
            for _ in 0..(k.max(0) as usize) {
                let _ = sc.get_uint();
                let _ = sc.get_uint();
            }
            continue;
        }
        let k = sc.get_uint();
        nodes[u].edge_size = k;
        let kk = k.max(0) as usize;
        for j in 0..kk {
            let to = sc.get_uint();
            let cost = sc.get_uint();
            if j < 100 {
                nodes[u].edge_to[j] = to;
                nodes[u].edge_cost[j] = cost;
            } else {
                // C would overflow fixed arrays (UB); ignore extras.
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
