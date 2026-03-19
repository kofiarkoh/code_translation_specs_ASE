use std::io::{self, Read};

const QMAX: usize = 150000;

#[derive(Clone, Copy)]
struct QUE {
    t: i32,
    r: i16,
    c: i16,
}

struct MinHeap {
    que: Vec<QUE>,
    qsize: usize,
}

impl MinHeap {
    fn new() -> Self {
        MinHeap {
            que: Vec::with_capacity(QMAX),
            qsize: 0,
        }
    }

    fn parent(i: usize) -> usize {
        i >> 1
    }

    fn left(i: usize) -> usize {
        i << 1
    }

    fn right(i: usize) -> usize {
        (i << 1) + 1
    }

    fn min_heapify(&mut self, i: usize) {
        let l = Self::left(i);
        let r = Self::right(i);
        let mut min = if l < self.qsize && self.que[l].t < self.que[i].t {
            l
        } else {
            i
        };
        if r < self.qsize && self.que[r].t < self.que[min].t {
            min = r;
        }
        if min != i {
            self.que.swap(i, min);
            self.min_heapify(min);
        }
    }

    fn deq(&mut self) {
        if self.qsize == 0 {
            return; // No elements to dequeue
        }
        self.que[0] = self.que[self.qsize - 1];
        self.qsize -= 1;
        self.min_heapify(0);
    }

    fn enq(&mut self, r: i16, c: i16, t: i32) {
        let i = self.qsize;
        self.qsize += 1;
        self.que.push(QUE { r, c, t });
        let mut i = i;
        while i > 0 && self.que[Self::parent(i)].t > self.que[i].t {
            self.que.swap(i, Self::parent(i));
            i = Self::parent(i);
        }
    }
}

fn in_number() -> i32 {
    let stdin = io::stdin();
    let mut n = 0;
    let mut buffer = [0; 1];
    let mut c = stdin.read(&mut buffer).unwrap();
    while c > 0 {
        let c = buffer[0] as char;
        if c.is_digit(10) {
            n = (n << 3) + (n << 1) + (c as i32 - '0' as i32);
        } else {
            break;
        }
        c = stdin.read(&mut buffer).unwrap();
    }
    n
}

fn main() {
    let mut l = [[0; 501]; 501];
    let mut level = [[0; 100001]; 2];
    let mut lsz = [0; 2];
    let mv: [(i16, i16); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut mk = [[0; 501]; 501];

    while let Some(R) = in_number().checked_sub(1) {
        for i in 0..2 {
            let W = in_number();
            let H = in_number();
            let X = in_number() - 1;
            let Y = in_number() - 1;

            for r in 0..H {
                for c in 0..W {
                    l[r as usize][c as usize] = in_number();
                }
            }

            mk = [[0; 501]; 501];
            let mut heap = MinHeap::new();
            heap.qsize = 0;
            level[i][0] = 0;
            lsz[i] = 1;
            heap.enq(Y, X, l[Y as usize][X as usize]);
            mk[Y as usize][X as usize] = 1;

            let mut max = 0;
            while heap.qsize > 0 {
                let QUE { t, r, c } = heap.que[0];
                heap.deq();
                if t > max {
                    max = t;
                }
                level[i][lsz[i]] = max;
                lsz[i] += 1;

                for j in 0..4 {
                    let r2 = r + mv[j].0;
                    let c2 = c + mv[j].1;
                    if r2 < 0 || r2 >= H as i16 || c2 < 0 || c2 >= W as i16 {
                        continue;
                    }
                    if mk[r2 as usize][c2 as usize] != 0 {
                        continue;
                    }
                    mk[r2 as usize][c2 as usize] = 1; // Mark as visited
                    heap.enq(r2, c2, l[r2 as usize][c2 as usize]);
                }
            }
        }

        let mut ans = i32::MAX;
        for i in 0..=R {
            let j = R - i;
            if i < lsz[0] && j < lsz[1] {
                let t = level[0][i] + level[1][j];
                if t < ans {
                    ans = t;
                }
            }
        }
        println!("{}", ans);
    }
}
