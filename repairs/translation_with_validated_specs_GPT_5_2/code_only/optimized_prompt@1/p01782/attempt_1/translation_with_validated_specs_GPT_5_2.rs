use std::io::{self, Read};

const MAX: usize = 105;

#[derive(Clone, Copy)]
struct T {
    a: [i8; 52],
}

static mut MX: [T; 10000] = [T { a: [0; 52] }; 10000];
static mut SZ: usize = 0;

const MX_ZERO: usize = 0;
const MX_INF: usize = 1;

fn mx_new() -> usize {
    unsafe {
        let r = SZ;
        SZ += 1;
        r
    }
}

fn mx_clear(idx: usize) {
    unsafe {
        MX[idx].a = [0; 52];
    }
}

fn mx_add1(a: usize, b: usize) {
    unsafe {
        for i in 0..52 {
            // C signed char arithmetic is promoted to int then converted back; emulate wrap.
            MX[a].a[i] = MX[a].a[i].wrapping_add(MX[b].a[i]);
        }
    }
}

fn mx_add2(s: usize, a: usize, b: usize) {
    unsafe {
        for i in 0..52 {
            MX[s].a[i] = MX[a].a[i].wrapping_add(MX[b].a[i]);
        }
    }
}

fn mx_mul2(m: usize, a: usize, k: i32) {
    unsafe {
        for i in 0..52 {
            let v = (MX[a].a[i] as i32).wrapping_mul(k);
            MX[m].a[i] = v as i8; // truncation/wrap like C conversion to signed char
        }
    }
}

fn mx_cpy(a: usize, b: usize) {
    unsafe {
        MX[a] = MX[b];
    }
}

fn mx_cmp(a: usize, b: usize) -> i32 {
    unsafe {
        for i in 0..52 {
            let k = (MX[a].a[i] as i32) - (MX[b].a[i] as i32);
            if k != 0 {
                return k;
            }
        }
        0
    }
}

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i32,
    cost: usize, // index into MX
}

static mut EDGE: [[Edge; MAX]; MAX] = [[Edge {
    to: 0,
    rev: 0,
    cap: 0,
    cost: 0,
}; MAX]; MAX];
static mut HI: [usize; MAX] = [0; MAX];

static mut V: usize = 0;
static mut DIST: [usize; MAX] = [0; MAX];
static mut PREVV: [usize; MAX] = [0; MAX];
static mut PREVE: [usize; MAX] = [0; MAX];

fn add_edge(from: usize, to: usize, cap: i32, cost: usize, rcost: usize) {
    unsafe {
        let f = HI[from];
        HI[from] += 1;
        let t = HI[to];
        HI[to] += 1;

        EDGE[from][f] = Edge {
            to,
            rev: t,
            cap,
            cost,
        };
        EDGE[to][t] = Edge {
            to: from,
            rev: f,
            cap: 0,
            cost: rcost,
        };
    }
}

fn min_cost_flow(s: usize, tnode: usize, mut f: i32) -> usize {
    let tmp = mx_new();
    unsafe {
        for i in 0..V {
            DIST[i] = mx_new();
        }
    }
    let ret = mx_new();

    while f > 0 {
        unsafe {
            for v in 0..V {
                mx_cpy(DIST[v], MX_INF);
            }
        }
        mx_clear(unsafe { DIST[s] });

        loop {
            let mut update = false;
            unsafe {
                for v in 0..V {
                    if mx_cmp(DIST[v], MX_INF) == 0 {
                        continue;
                    }
                    for i in 0..HI[v] {
                        let e = EDGE[v][i];
                        if e.cap <= 0 {
                            continue;
                        }
                        let nv = e.to;
                        mx_add2(tmp, DIST[v], e.cost);
                        if mx_cmp(DIST[nv], tmp) > 0 {
                            mx_cpy(DIST[nv], tmp);
                            PREVV[nv] = v;
                            PREVE[nv] = i;
                            update = true;
                        }
                    }
                }
            }
            if !update {
                break;
            }
        }

        let mut d = f;
        let mut v = tnode;
        while v != s {
            unsafe {
                let pv = PREVV[v];
                let pe = PREVE[v];
                let e = EDGE[pv][pe];
                if d > e.cap {
                    d = e.cap;
                }
                v = pv;
            }
        }
        f -= d;

        mx_mul2(tmp, unsafe { DIST[tnode] }, d as i32);
        mx_add1(ret, tmp);

        let mut v2 = tnode;
        while v2 != s {
            unsafe {
                let pv = PREVV[v2];
                let pe = PREVE[v2];
                let rev = EDGE[pv][pe].rev;

                EDGE[pv][pe].cap -= d;
                EDGE[v2][rev].cap += d;

                v2 = pv;
            }
        }
    }

    ret
}

fn main() {
    // Read entire stdin; C uses fgets/atoi and then reads N lines.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap_or(0),
        None => return,
    };

    let mut map: Vec<Vec<u8>> = Vec::with_capacity(n);
    for _ in 0..n {
        let s = it.next().unwrap_or("");
        map.push(s.as_bytes().to_vec());
    }

    // Initialize mx_inf to 60 in all 52 positions.
    unsafe {
        for i in 0..52 {
            MX[MX_INF].a[i] = 60;
        }
        SZ = 2; // mx_zero=0, mx_inf=1 already reserved
    }

    let source = n << 1;
    let sink = source + 1;
    unsafe {
        V = sink + 1;
        HI = [0; MAX];
    }

    for i in 0..n {
        add_edge(source, i, 1, mx_new(), mx_new());
        for j in 0..n {
            let c = *map[i].get(j).unwrap_or(&b'A');
            let a = mx_new();
            let b = mx_new();
            unsafe {
                if c < b'a' {
                    let idx = (c - b'A') as usize;
                    MX[a].a[idx] = -1;
                    MX[b].a[idx] = 1;
                } else {
                    let idx = 26 + (c - b'a') as usize;
                    MX[a].a[idx] = -1;
                    MX[b].a[idx] = 1;
                }
            }
            add_edge(i, n + j, 1, a, b);
        }
        add_edge(n + i, sink, 1, mx_new(), mx_new());
    }

    let ans = min_cost_flow(source, sink, n as i32);

    let mut out = String::new();
    unsafe {
        for i in 0..26 {
            let mut b = -(MX[ans].a[i] as i32);
            while b > 0 {
                out.push((b'A' + i as u8) as char);
                b -= 1;
            }
        }
        for i in 0..26 {
            let mut b = -(MX[ans].a[i + 26] as i32);
            while b > 0 {
                out.push((b'a' + i as u8) as char);
                b -= 1;
            }
        }
    }
    out.push('\n');
    print!("{out}");
}
