use std::io::{self, Read};

const MAX_V: usize = 10002;

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

    // Emulates the C in(): reads a non-negative integer by consuming bytes and
    // stopping when the next byte is < '0'. Assumes input is well-formed
    // (as the C code does), i.e., starts at a digit when called.
    fn in_u32(&mut self) -> u32 {
        let mut n: u32 = 0;
        // In the original C, getchar_unlocked() is called immediately and then
        // the loop runs at least once. We mirror that behavior.
        let mut c = self.buf[self.idx];
        self.idx += 1;
        loop {
            n = (n << 3).wrapping_add(n << 1).wrapping_add((c & 0x0f) as u32);
            if self.idx >= self.buf.len() {
                break;
            }
            c = self.buf[self.idx];
            self.idx += 1;
            if c < b'0' {
                break;
            }
        }
        n
    }
}

fn main() {
    let mut sc = Scanner::new();

    let v = sc.in_u32() as usize;
    let mut e = sc.in_u32() as usize;

    // C uses fixed-size globals sized by MAX_V; we mirror that capacity.
    // Only indices < V are used.
    let mut hi = vec![0usize; MAX_V];
    let mut count = vec![0i32; MAX_V];

    // memo stores pairs (s,t) as in the C code.
    let mut memo: Vec<usize> = Vec::with_capacity(e * 2);

    for _ in 0..e {
        let s = sc.in_u32() as usize;
        let t = sc.in_u32() as usize;
        memo.push(s);
        memo.push(t);
        hi[s] += 1;
    }

    // Allocate adjacency lists with exact sizes (like malloc(hi[i]<<2)).
    let mut to: Vec<Vec<usize>> = vec![Vec::new(); MAX_V];
    for i in 0..v {
        if hi[i] != 0 {
            to[i] = Vec::with_capacity(hi[i]);
        }
    }

    // memset(hi, 0, V<<2);
    for i in 0..v {
        hi[i] = 0;
    }

    // Fill adjacency and indegree counts.
    let mut j = 0usize;
    while e > 0 {
        let s = memo[j];
        let t = memo[j + 1];
        j += 2;

        to[s].push(t);
        hi[s] += 1;
        count[t] += 1;

        e -= 1;
    }

    // topological_sort(): Kahn's algorithm with queue array q[MAX_V].
    let mut q = vec![0usize; MAX_V];
    let mut top = 0usize;
    let mut end = 0usize;

    for i in 0..v {
        if count[i] == 0 {
            q[end] = i;
            end += 1;
        }
    }

    let mut out = String::new();
    while top < end {
        let i = q[top];
        top += 1;

        out.push_str(&format!("{i}\n"));

        // Iterate over outgoing edges; hi[i] equals to[i].len() here.
        for &k in to[i].iter() {
            count[k] -= 1;
            if count[k] == 0 {
                q[end] = k;
                end += 1;
            }
        }
    }

    print!("{out}");
}
