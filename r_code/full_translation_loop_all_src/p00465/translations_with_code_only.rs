use std::io::{self, Read};

const QMAX: usize = 150000;

#[derive(Clone, Copy, Default)]
struct Que {
    t: i32,
    r: i16,
    c: i16,
}

struct MinHeap {
    a: [Que; QMAX],
    size: usize,
}

impl MinHeap {
    fn new() -> Self {
        Self {
            a: [Que::default(); QMAX],
            size: 0,
        }
    }

    #[inline]
    fn parent(i: usize) -> usize {
        i >> 1
    }
    #[inline]
    fn left(i: usize) -> usize {
        i << 1
    }
    #[inline]
    fn right(i: usize) -> usize {
        (i << 1) + 1
    }

    fn min_heapify(&mut self, mut i: usize) {
        loop {
            let l = Self::left(i);
            let r = Self::right(i);
            let mut min = i;

            if l < self.size && self.a[l].t < self.a[i].t {
                min = l;
            }
            if r < self.size && self.a[r].t < self.a[min].t {
                min = r;
            }
            if min == i {
                break;
            }
            self.a.swap(i, min);
            i = min;
        }
    }

    fn deq(&mut self) {
        // C code assumes size > 0 when called.
        self.size -= 1;
        self.a[0] = self.a[self.size];
        self.min_heapify(0);
    }

    fn enq(&mut self, r: i32, c: i32, t: i32) {
        let mut i = self.size;
        self.size += 1;
        self.a[i] = Que {
            t,
            r: r as i16,
            c: c as i16,
        };
        while i > 0 {
            let p = Self::parent(i);
            if self.a[p].t <= self.a[i].t {
                break;
            }
            self.a.swap(i, p);
            i = p;
        }
    }
}

// Fast input matching the C in(): reads digits, stops when next char < '0'.
// It does not skip non-digits before the number; the original AOJ input is well-formed.
struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).unwrap();
        Self {
            buf: s.into_bytes(),
            idx: 0,
        }
    }
    fn in_i32(&mut self) -> i32 {
        let mut n: i32 = 0;
        if self.idx >= self.buf.len() {
            return 0;
        }
        let mut c = self.buf[self.idx];
        self.idx += 1;
        loop {
            n = (n << 3) + (n << 1) + ((c & 0x0f) as i32);
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
    const MV: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    // Static-sized arrays as in C.
    let mut l = vec![vec![0i32; 501]; 501];
    let mut mk = vec![vec![0u8; 501]; 501];
    let mut level = vec![vec![0i32; 100001]; 2];
    let mut lsz = [0usize; 2];

    let mut sc = Scanner::new();
    let mut out = String::new();

    loop {
        let r_in = sc.in_i32();
        if r_in == 0 {
            break;
        }
        let r_limit = r_in as usize;

        for side in 0..2 {
            let w = sc.in_i32() as i32;
            let h = sc.in_i32() as i32;
            let x = sc.in_i32() - 1;
            let y = sc.in_i32() - 1;

            for rr in 0..h {
                for cc in 0..w {
                    l[rr as usize][cc as usize] = sc.in_i32();
                }
            }

            // memset(mk, 0, sizeof(mk))
            for rr in 0..(h as usize) {
                mk[rr][..(w as usize)].fill(0);
            }

            let mut heap = MinHeap::new();
            let mut maxv: i32 = 0;
            level[side][0] = 0;
            lsz[side] = 1;

            heap.enq(y, x, l[y as usize][x as usize]);
            mk[y as usize][x as usize] = 1;

            while heap.size != 0 {
                let top = heap.a[0];
                let t = top.t;
                let rr = top.r as i32;
                let cc = top.c as i32;
                heap.deq();

                if t > maxv {
                    maxv = t;
                }
                let idx = lsz[side];
                level[side][idx] = maxv;
                lsz[side] += 1;

                for (dr, dc) in MV.iter() {
                    let r2 = rr + dr;
                    let c2 = cc + dc;
                    if r2 < 0 || r2 >= h || c2 < 0 || c2 >= w {
                        continue;
                    }
                    let ur2 = r2 as usize;
                    let uc2 = c2 as usize;
                    if mk[ur2][uc2] != 0 {
                        continue;
                    }
                    // Same room is never enqueued twice.
                    mk[ur2][uc2] = 1;
                    heap.enq(r2, c2, l[ur2][uc2]);
                }
            }
        }

        let mut ans: i32 = 0x7ffffff; // matches C literal
        for i in 0..=r_limit {
            let j = r_limit - i;
            if i < lsz[0] && j < lsz[1] {
                let t = level[0][i] + level[1][j];
                if t < ans {
                    ans = t;
                }
            }
        }
        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
