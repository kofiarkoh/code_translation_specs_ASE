// AOJ 3055 Don't Burn Kotatsu Turtle
// 2019.4.7 bal4u

use std::io::{self, BufRead};

const MAX: usize = 500000;
const MAXV: usize = 1005;

#[derive(Clone, Copy)]
struct QUE {
    t: i64,
    r: usize,
    c: usize,
}

struct MinHeap {
    que: Vec<QUE>,
    qsize: usize,
}

impl MinHeap {
    fn new() -> Self {
        MinHeap {
            que: Vec::with_capacity(MAX),
            qsize: 0,
        }
    }

    fn min_heapify(&mut self, i: usize) {
        let mut min = i;
        let l = self.left(i);
        let r = self.right(i);

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
        if self.qsize > 0 {
            self.que[0] = self.que[self.qsize - 1];
            self.qsize -= 1;
            self.min_heapify(0);
        }
    }

    fn enq(&mut self, r: usize, c: usize, t: i64) {
        let i = self.qsize;
        self.qsize += 1;
        self.que.push(QUE { t, r, c });

        let mut i = i;
        while i > 0 {
            let parent = self.parent(i);
            if self.que[parent].t > self.que[i].t {
                self.que.swap(i, parent);
                i = parent;
            } else {
                break;
            }
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
    for &c in bytes {
        if c <= b' ' {
            break;
        }
        s[i] = c;
        i += 1;
    }
    s[i] = 0; // Null-terminate the string
}

fn dijkstra(sr: usize, sc: usize, gr: usize, gc: usize, map: &Vec<Vec<u8>>, bomb: &Vec<Vec<u8>>, tt: &[[i64; 2]; 8], h: usize, w: usize) -> i64 {
    let mut vis = vec![vec![0; MAXV]; MAXV];
    let mut heap = MinHeap::new();
    heap.enq(sr, sc, 0);

    while heap.qsize > 0 {
        let QUE { r, c, t } = heap.que[0];
        heap.deq();

        if r == gr && c == gc {
            return t;
        }
        if vis[r][c] != 0 {
            continue;
        }
        vis[r][c] = 1;

        let f = bomb[r][c];
        let k = (2 - f) << 2;

        for i in 0..k {
            let rr = r.wrapping_add(mv[i][0] as usize);
            let cc = c.wrapping_add(mv[i][1] as usize);
            if rr < 1 || rr > h || cc < 1 || cc > w {
                continue;
            }
            let g = (map[rr][cc] == b'#') as i64;
            if f != 0 && g != 0 {
                continue;
            }
            if (i < 4 || g != 0) && vis[rr][cc] == 0 {
                heap.enq(rr, cc, t + tt[i][g as usize]);
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

    let mut map = vec![vec![0; MAXV]; MAXV];
    let mut bomb = vec![vec![0; MAXV]; MAXV];
    let mut sr = 0;
    let mut sc = 0;
    let mut tr = 0;
    let mut tc = 0;

    for r in 1..=h {
        let mut line = vec![0; MAXV];
        ins(&mut line);
        for c in 1..=w {
            map[r][c] = line[c];
            if map[r][c] == b'*' {
                bomb[r][c] = 1;
                for i in 0..8 {
                    bomb[(r as isize + mv[i][0]) as usize][(c as isize + mv[i][1]) as usize] = 1;
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

    let ans = dijkstra(sr, sc, tr, tc, &map, &bomb, &tt, h, w);
    if ans >= 0 {
        println!("{}", ans);
    } else {
        println!("INF");
    }
}

const mv: [[isize; 2]; 8] = [
    [-1, 0], [0, 1], [1, 0], [0, -1],
    [-1, -1], [-1, 1], [1, 1], [1, -1],
];
