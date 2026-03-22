use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

const HASHSIZ: usize = 29989;
const MAX: usize = 6002;
// C uses dist[MAX][2002]; cap = 10*in(), so cap is expected <= 2001.
const CAP_MAX: usize = 2002;

#[derive(Clone, Copy)]
struct HashEnt {
    // Store index into `cities` (string table). None means empty.
    idx: Option<usize>,
    id: i16,
}

fn hash_key(s: &str) -> usize {
    // Emulate C: unsigned long long i; i = (i<<6) + *p++;
    // Use wrapping arithmetic to match modulo 2^64 behavior.
    let mut i: u64 = 0;
    for &b in s.as_bytes() {
        i = i.wrapping_shl(6).wrapping_add(b as u64);
    }
    (i % (HASHSIZ as u64)) as usize
}

// Returns existing id if found, else inserts and returns -1 (like C).
fn insert(hash: &mut [HashEnt; HASHSIZ + 2], cities: &[String], s_idx: usize, id: i16) -> i16 {
    let s = &cities[s_idx];
    let mut tp = hash_key(s);
    let hashend = HASHSIZ;
    loop {
        match hash[tp].idx {
            Some(existing_idx) => {
                if cities[existing_idx] == *s {
                    return hash[tp].id;
                }
                tp += 1;
                if tp == hashend {
                    tp = 0;
                }
            }
            None => {
                hash[tp].idx = Some(s_idx);
                hash[tp].id = id;
                return -1;
            }
        }
    }
}

struct Scanner {
    buf: Vec<u8>,
    pos: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self { buf: input.into_bytes(), pos: 0 }
    }

    fn getc(&mut self) -> Option<u8> {
        if self.pos >= self.buf.len() {
            None
        } else {
            let c = self.buf[self.pos];
            self.pos += 1;
            Some(c)
        }
    }

    // Emulate C in(): reads digits, assumes first char is a digit.
    // Stops when next char < '0'. Returns 0 if EOF before any digit.
    fn in_int(&mut self) -> i32 {
        let mut n: i32 = 0;
        let mut c = match self.getc() {
            Some(x) => x,
            None => return 0,
        };
        // C does: do { ... } while (c >= '0');
        // If input is well-formed, first c is digit. We keep same structure.
        loop {
            n = (n << 3) + (n << 1) + ((c & 0x0f) as i32);
            c = match self.getc() {
                Some(x) => x,
                None => break,
            };
            if c < b'0' {
                break;
            }
        }
        n
    }

    // Emulate C ins(): reads a token (bytes > ' '), then terminates.
    // Returns empty string on EOF.
    fn ins_token(&mut self) -> String {
        let mut out: Vec<u8> = Vec::new();
        loop {
            let c = match self.getc() {
                Some(x) => x,
                None => break,
            };
            if c <= b' ' {
                break;
            }
            out.push(c);
        }
        String::from_utf8(out).unwrap_or_default()
    }
}

fn dijkstra(
    sz: usize,
    cap: usize,
    start: usize,
    goal: usize,
    lpg: &[bool],
    adj: &[Vec<(usize, i16)>],
) -> i32 {
    // dist initialized to 0x10101010 in C via memset(0x10) on int array.
    // That value is 0x10101010 = 269_488_144.
    const INF: i32 = 0x1010_1010;

    // dist[city][fuel]
    let mut dist = vec![vec![INF; CAP_MAX]; sz];

    let mut heap: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
    if cap >= CAP_MAX {
        // C would write out of bounds; we clamp to preserve memory safety.
        // This is a best-effort emulation; typical AOJ constraints keep cap < CAP_MAX.
        return -1;
    }
    dist[start][cap] = 0;
    heap.push((Reverse(0), start, cap));

    while let Some((Reverse(d), s, mut c)) = heap.pop() {
        if s == goal {
            return d;
        }
        if dist[s][c] < d {
            continue;
        }
        if lpg[s] {
            c = cap;
        }
        for &(e, w) in &adj[s] {
            let w_i32 = w as i32;
            let nc_i32 = (c as i32) - w_i32;
            if nc_i32 < 0 {
                continue;
            }
            let nc = nc_i32 as usize;
            let nd = d + w_i32;
            if nd < dist[e][nc] {
                dist[e][nc] = nd;
                heap.push((Reverse(nd), e, nc));
            }
        }
    }
    -1
}

fn main() {
    let mut sc = Scanner::new();
    let mut out = String::new();

    loop {
        let n = sc.in_int();
        if n == 0 {
            break;
        }
        let m = sc.in_int();
        let cap = (10 * sc.in_int()) as usize;

        let mut hash = [HashEnt { idx: None, id: 0 }; HASHSIZ + 2];

        let mut cities: Vec<String> = Vec::with_capacity(MAX);
        cities.push(sc.ins_token()); // city[0]
        cities.push(sc.ins_token()); // city[1]
        // Insert start/goal
        insert(&mut hash, &cities, 0, 0);
        insert(&mut hash, &cities, 1, 1);

        let mut hi = vec![0usize; MAX];
        let mut memo: Vec<i32> = Vec::with_capacity((n as usize) * 3);

        let mut sz: usize = 2;
        for _ in 0..n {
            // first city
            let s1 = sc.ins_token();
            cities.push(s1);
            let idx1 = cities.len() - 1;
            let mut a = insert(&mut hash, &cities, idx1, sz as i16);
            let a_usize: usize;
            if a < 0 {
                a_usize = sz;
                sz += 1;
            } else {
                a_usize = a as usize;
            }
            memo.push(a_usize as i32);
            hi[a_usize] += 1;

            // second city
            let s2 = sc.ins_token();
            cities.push(s2);
            let idx2 = cities.len() - 1;
            let mut b = insert(&mut hash, &cities, idx2, sz as i16);
            let b_usize: usize;
            if b < 0 {
                b_usize = sz;
                sz += 1;
            } else {
                b_usize = b as usize;
            }
            memo.push(b_usize as i32);
            hi[b_usize] += 1;

            let c = sc.in_int();
            memo.push(c);
        }

        // Build adjacency lists (undirected), using hi counts as degrees.
        let mut adj: Vec<Vec<(usize, i16)>> = vec![Vec::new(); sz];
        for v in 0..sz {
            if hi[v] > 0 {
                adj[v].reserve(hi[v]);
            }
        }
        // Fill edges
        let mut j = 0usize;
        for _ in 0..n {
            let a = memo[j] as usize;
            let b = memo[j + 1] as usize;
            let c = memo[j + 2] as i16;
            j += 3;
            adj[a].push((b, c));
            adj[b].push((a, c));
        }

        // LPG flags
        let mut lpg = vec![false; sz];
        for _ in 0..m {
            let name = sc.ins_token();
            cities.push(name);
            let idx = cities.len() - 1;
            let a = insert(&mut hash, &cities, idx, -1);
            if a >= 0 {
                lpg[a as usize] = true;
            }
        }

        let ans = dijkstra(sz, cap, 0, 1, &lpg, &adj);
        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
