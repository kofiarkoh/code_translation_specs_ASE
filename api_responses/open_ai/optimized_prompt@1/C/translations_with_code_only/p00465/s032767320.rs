// AOJ 0542: Authentication Level
// 2018.1.28 bal4u

use std::io::{self, BufRead};

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
        if self.qsize == 0 {
            return; // No elements to dequeue
        }
        self.qsize -= 1;
        if self.qsize > 0 {
            self.que[0] = self.que[self.qsize];
            self.min_heapify(0);
        }
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
    let mut input = stdin.lock().lines().next().unwrap().unwrap();
    let mut n = 0;

    while let Some(c) = input.chars().next() {
        if c.is_digit(10) {
            n = (n << 3) + (n << 1) + (c.to_digit(10).unwrap() as i32);
            input.remove(0);
        } else {
            break;
        }
    }
    n
}

fn main() {
    let mut l = vec![vec![0; 501]; 501];
    let mut level = vec![vec![0; 100001], vec![0; 100001]];
    let mut lsz = [0, 0];
    let mv: [(i16, i16); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut mk = vec![vec![0; 501]; 501];

    let mut input = io::stdin().lock().lines();
    while let Ok(r) = input.next().unwrap().unwrap().parse::<i32>() {
        for i in 0..2 {
            let w = in_number();
            let h = in_number();
            let x = in_number() - 1;
            let y = in_number() - 1;

            for r in 0..h {
                for c in 0..w {
                    l[r as usize][c as usize] = in_number();
                }
            }

            mk.iter_mut().for_each(|row| row.fill(0));
            let mut heap = MinHeap::new();
            heap.qsize = 0;
            level[i][0] = 0;
            lsz[i] = 1;
            heap.enq(y, x, l[y as usize][x as usize]);
            mk[y as usize][x as usize] = 1;

            while heap.qsize > 0 {
                let t = heap.que[0].t;
                let r = heap.que[0].r;
                let c = heap.que[0].c;
                heap.deq();

                level[i][lsz[i]] = t;
                lsz[i] += 1;

                for j in 0..4 {
                    let r2 = r + mv[j].0;
                    let c2 = c + mv[j].1;
                    if r2 < 0 || r2 >= h || c2 < 0 || c2 >= w {
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
        for i in 0..=r {
            let j = r - i;
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
