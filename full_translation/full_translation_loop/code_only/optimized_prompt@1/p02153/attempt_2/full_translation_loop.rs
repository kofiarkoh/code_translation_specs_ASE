use std::io::{self, BufRead};

const MAX: usize = 500_000;
const MAXV: usize = 1005;

#[derive(Clone, Copy)]
struct Que {
    t: i64,
    r: usize,
    c: usize,
}

struct MinHeap {
    que: Vec<Que>,
    qsize: usize,
}

impl MinHeap {
    fn new() -> Self {
        MinHeap {
            que: Vec::with_capacity(MAX),
            qsize: 0,
        }
    }

    fn parent(&self, i: usize) -> usize {
        i >> 1
    }

    fn left(&self, i: usize) -> usize {
        i << 1
    }

    fn right(&self, i: usize) -> usize {
        (i << 1) + 1
    }

    fn min_heapify(&mut self, i: usize) {
        let l = self.left(i);
        let r = self.right(i);
        let mut min = i;

        if l < self.qsize && self.que[l].t < self.que[min].t {
            min = l;
        }
        if r < self.qsize && self.que[r].t < self.que[min].t {
            min = r;
        }
        if min != i {
            self.que.swap(i, min);
            self.min_heapify(min);
        }
    }

    fn deq(&mut self) {
        self.que[0] = self.que[self.qsize - 1];
        self.qsize -= 1;
        self.min_heapify(0);
    }

    fn enq(&mut self, r: usize, c: usize, t: i64) {
        let i = self.qsize;
        self.qsize += 1;
        self.que.push(Que { t, r, c });
        let mut i = i;

        while i > 0 && self.que[self.parent(i)].t > self.que[i].t {
            self.que.swap(i, self.parent(i));
            i = self.parent(i);
        }
    }
}

fn in_non_negative() -> usize {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn ins(s: &mut [u8]) {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() && bytes[i] > b' ' {
        s[i] = bytes[i];
        i += 1;
    }
    s[i] = 0; // Null-terminate the string
}

fn dijkstra(
    heap: &mut MinHeap,
    vis: &mut [[bool; MAXV]; MAXV],
    map: &[[u8; MAXV]; MAXV],
    bomb: &[[u8; MAXV]; MAXV],
    tt: &[[i64; 2]; 8],
    sr: usize,
    sc: usize,
    tr: usize,
    tc: usize,
    h: usize,
    w: usize,
) -> i64 {
    let mv: [(isize, isize); 8] = [
        (-1, 0), (0, 1), (1, 0), (0, -1),
        (-1, -1), (-1, 1), (1, 1), (1, -1),
    ];

    heap.enq(sr, sc, 0);
    while heap.qsize > 0 {
        let Que { r, c, t } = heap.que[0];
        heap.deq();

        if r == tr && c == tc {
            return t;
        }
        if vis[r][c] {
            continue;
        }
        vis[r][c] = true;

        let f = bomb[r][c];
        let k = (2 - f) << 2;

        for i in 0..k {
            let rr = (r as isize + mv[i as usize].0) as usize;
            let cc = (c as isize + mv[i as usize].1) as usize;

            if rr < 1 || rr > h || cc < 1 || cc > w {
                continue;
            }
            let g = (map[rr][cc] == b'#') as usize;
            if f != 0 && g != 0 {
                continue;
            }
            if (i < 4 || g != 0) && !vis[rr][cc] {
                heap.enq(rr, cc, t + tt[i as usize][g]);
            }
        }
    }
    -1
}

fn main() {
    let h = in_non_negative();
    let w = in_non_negative();
    let a = in_non_negative();
    let b = in_non_negative();

    let mut map = [[0; MAXV]; MAXV];
    let mut bomb = [[0; MAXV]; MAXV];
    let mut vis = [[false; MAXV]; MAXV];
    let mut sr = 0;
    let mut sc = 0;
    let mut tr = 0;
    let mut tc = 0;

    for r in 1..=h {
        let mut line = [0; MAXV];
        ins(&mut line);
        for c in 1..=w {
            map[r][c] = line[c];
            if map[r][c] == b'*' {
                bomb[r][c] = 1;
                for i in 0..8 {
                    let rr = r as isize + mv[i].0;
                    let cc = c as isize + mv[i].1;
                    if rr >= 1 && rr <= h as isize && cc >= 1 && cc <= w as isize {
                        bomb[rr as usize][cc as usize] = 1;
                    }
                }
            } else if map[r][c] == b's' {
                sr = r;
                sc = c;
            } else if map[r][c] == b'g' {
                tr = r;
                tc = c;
            }
        }
    }

    let mut tt = [[0; 2]; 8];
    for i in 0..4 {
        tt[i][0] = a as i64;
        tt[i][1] = (a + b) as i64;
    }
    let a = a << 1;
    for i in 4..8 {
        tt[i][0] = a as i64;
        tt[i][1] = (a + b) as i64;
    }

    let mut heap = MinHeap::new();
    let ans = dijkstra(&mut heap, &mut vis, &map, &bomb, &tt, sr, sc, tr, tc, h, w);
    if ans >= 0 {
        println!("{}", ans);
    } else {
        println!("INF");
    }
}
