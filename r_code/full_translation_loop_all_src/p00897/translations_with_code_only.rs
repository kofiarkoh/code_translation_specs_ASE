use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Que {
    t: i32,
    s: usize,
    c: usize,
}

// A fast byte reader to mimic getchar_unlocked()-style parsing.
struct Reader {
    buf: Vec<u8>,
    idx: usize,
}
impl Reader {
    fn new() -> Self {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        Self { buf: input, idx: 0 }
    }
    fn getc(&mut self) -> u8 {
        if self.idx >= self.buf.len() {
            0
        } else {
            let c = self.buf[self.idx];
            self.idx += 1;
            c
        }
    }
    // Mimics C in(): reads a non-negative integer, assuming the next char is a digit.
    // Stops when c < '0'. (Same as the C code; it does not skip whitespace.)
    fn in_int(&mut self) -> i32 {
        let mut n: i32 = 0;
        let mut c = self.getc();
        loop {
            n = n
                .wrapping_shl(3)
                .wrapping_add(n.wrapping_shl(1))
                .wrapping_add((c & 0x0f) as i32);
            c = self.getc();
            if c < b'0' {
                break;
            }
        }
        n
    }
    // Mimics C ins(): reads a token until <= ' ' and NUL-terminates in C.
    fn ins_token(&mut self) -> Vec<u8> {
        let mut v = Vec::new();
        loop {
            let c = self.getc();
            if c <= b' ' {
                break;
            }
            v.push(c);
        }
        v
    }
}

fn dijkstra(
    cap: usize,
    start: usize,
    goal: usize,
    to: &Vec<Vec<usize>>,
    dd: &Vec<Vec<i32>>,
    lpg: &Vec<bool>,
) -> i32 {
    let n = to.len();
    // C uses dist[MAX][2002] and memset(0x10). Here we size by (n, cap+1).
    // Use a large sentinel similar in spirit.
    let inf: i32 = 0x10101010;
    let mut dist = vec![vec![inf; cap + 1]; n];

    // Custom binary min-heap to preserve the C heap behavior closely.
    let mut heap: Vec<Que> = Vec::new();
    let mut enq = |heap: &mut Vec<Que>, s: usize, t: i32, c: usize| {
        heap.push(Que { s, t, c });
        let mut i = heap.len() - 1;
        while i > 0 {
            let p = (i >> 1);
            if heap[p].t <= heap[i].t {
                break;
            }
            heap.swap(i, p);
            i = p;
        }
    };
    let mut min_heapify = |heap: &mut Vec<Que>, mut i: usize| {
        loop {
            let l = i << 1;
            let r = (i << 1) + 1;
            let mut m = i;
            if l < heap.len() && heap[l].t < heap[m].t {
                m = l;
            }
            if r < heap.len() && heap[r].t < heap[m].t {
                m = r;
            }
            if m == i {
                break;
            }
            heap.swap(i, m);
            i = m;
        }
    };
    let mut deq = |heap: &mut Vec<Que>| {
        let last = heap.pop().unwrap();
        if !heap.is_empty() {
            heap[0] = last;
            min_heapify(heap, 0);
        }
    };

    enq(&mut heap, start, 0, cap);
    dist[start][cap] = 0;

    while !heap.is_empty() {
        let Que { s, t: d, c } = heap[0];
        deq(&mut heap);

        if s == goal {
            return d;
        }
        if dist[s][c] < d {
            continue;
        }

        let mut fuel = c;
        if lpg[s] {
            fuel = cap;
        }

        for i in 0..to[s].len() {
            let e = to[s][i];
            let w = dd[s][i];
            let nc_i32 = fuel as i32 - w;
            if nc_i32 < 0 {
                continue;
            }
            let nc = nc_i32 as usize;
            let nd = d.wrapping_add(w);
            if dist[e][nc] > nd {
                dist[e][nc] = nd;
                enq(&mut heap, e, nd, nc);
            }
        }
    }
    -1
}

fn main() {
    let mut rd = Reader::new();
    let mut out = String::new();

    loop {
        let n_edges = rd.in_int();
        if n_edges == 0 {
            break;
        }
        let m = rd.in_int();
        let cap = (10 * rd.in_int()) as usize;

        // Read start and goal city names.
        let c0 = rd.ins_token();
        let c1 = rd.ins_token();

        // Hash table in C stores pointers to city strings; here we map by bytes.
        // insert() returns existing id or -1 if newly inserted.
        let mut map: HashMap<Vec<u8>, usize> = HashMap::new();
        let mut next_id: usize = 0;

        let mut insert = |map: &mut HashMap<Vec<u8>, usize>, key: Vec<u8>, id: usize| -> i32 {
            if let Some(&v) = map.get(&key) {
                v as i32
            } else {
                map.insert(key, id);
                -1
            }
        };

        insert(&mut map, c0.clone(), 0);
        insert(&mut map, c1.clone(), 1);
        next_id = 2;

        // First pass: collect edges and degrees (hi[] in C).
        let mut memo_a: Vec<usize> = Vec::with_capacity(n_edges as usize);
        let mut memo_b: Vec<usize> = Vec::with_capacity(n_edges as usize);
        let mut memo_c: Vec<i32> = Vec::with_capacity(n_edges as usize);

        // We don't know sz upfront; grow degrees as needed.
        let mut deg: Vec<usize> = vec![0; 2];

        for _ in 0..n_edges {
            let name_a = rd.ins_token();
            let mut a = insert(&mut map, name_a.clone(), next_id);
            let a_id: usize;
            if a < 0 {
                a_id = next_id;
                next_id += 1;
            } else {
                a_id = a as usize;
            }
            if a_id >= deg.len() {
                deg.resize(a_id + 1, 0);
            }
            deg[a_id] += 1;

            let name_b = rd.ins_token();
            let mut b = insert(&mut map, name_b.clone(), next_id);
            let b_id: usize;
            if b < 0 {
                b_id = next_id;
                next_id += 1;
            } else {
                b_id = b as usize;
            }
            if b_id >= deg.len() {
                deg.resize(b_id + 1, 0);
            }
            deg[b_id] += 1;

            let c = rd.in_int();

            memo_a.push(a_id);
            memo_b.push(b_id);
            memo_c.push(c);
        }

        let sz = next_id;

        // Build adjacency lists with exact sizes (like malloc(hi[a] << 1) for short arrays in C).
        // In C, to/dd are short arrays; here use usize/i32.
        let mut to: Vec<Vec<usize>> = vec![Vec::new(); sz];
        let mut dd: Vec<Vec<i32>> = vec![Vec::new(); sz];
        for v in 0..sz {
            let d = if v < deg.len() { deg[v] } else { 0 };
            to[v].reserve(d);
            dd[v].reserve(d);
        }

        for i in 0..(n_edges as usize) {
            let a = memo_a[i];
            let b = memo_b[i];
            let c = memo_c[i];
            to[a].push(b);
            dd[a].push(c);
            to[b].push(a);
            dd[b].push(c);
        }

        // LPG flags: C does memset(LPG,0,sz) then sets for known cities only.
        let mut lpg = vec![false; sz];
        for _ in 0..m {
            let name = rd.ins_token();
            // C calls insert(buf, -1) and uses returned id if exists; otherwise ignores.
            if let Some(&id) = map.get(&name) {
                if id < lpg.len() {
                    lpg[id] = true;
                }
            }
        }

        let ans = dijkstra(cap, 0, 1, &to, &dd, &lpg);
        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
