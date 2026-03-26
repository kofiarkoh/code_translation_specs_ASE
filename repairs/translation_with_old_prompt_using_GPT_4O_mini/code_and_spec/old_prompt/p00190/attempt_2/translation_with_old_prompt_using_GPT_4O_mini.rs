use std::collections::HashMap;
use std::io::{self, BufRead};

const MAX: usize = 500000;
const HASHSIZ: usize = 900001;

type UL = u64;

struct QueueEntry {
    state: UL,
    b1: i8,
    b2: i8,
}

struct PuzzleSolver {
    hash_table: HashMap<UL, i8>,
    queue: Vec<QueueEntry>,
    top: usize,
    end: usize,
}

impl PuzzleSolver {
    fn new() -> Self {
        Self {
            hash_table: HashMap::new(),
            queue: Vec::with_capacity(MAX),
            top: 0,
            end: 0,
        }
    }

    fn lookup(&mut self, state: UL) -> Option<i8> {
        self.hash_table.get(&state).copied()
    }

    fn insert(&mut self, state: UL, moves: i8) -> bool {
        if self.hash_table.contains_key(&state) {
            return false;
        }
        self.hash_table.insert(state, moves);
        true
    }

    fn swap(&self, state: UL, p1: usize, sp: usize) -> UL {
        let n1 = (state >> (4 * (12 - p1))) & 0xf;
        let mut new_state = state & !(0xf << (4 * (12 - p1)));
        new_state |= n1 << (4 * (12 - sp));
        new_state
    }

    fn sach(&mut self, state: UL, b1: i8, b2: i8, mode: i8, step: i8, lim: i8) -> i8 {
        self.queue.clear();
        self.top = 0;
        self.end = 0;

        self.queue.push(QueueEntry { state, b1, b2 });
        self.end += 1;

        while self.top < self.end {
            let current = &self.queue[self.top];
            let (s, b) = (current.state, [current.b1, current.b2]);
            self.top += 1;

            if s == 0 {
                return step + 1;
            }
            if step > lim {
                break;
            }

            for i in 0..2 {
                for &x in &MOVE[b[i] as usize] {
                    if x == b[0] || x == b[1] {
                        continue;
                    }
                    let s2 = self.swap(s, x as usize, b[i] as usize);
                    if mode == 1 {
                        if self.lookup(s2).is_some() {
                            return step;
                        }
                    } else if !self.insert(s2, step) {
                        continue;
                    }
                    self.queue.push(QueueEntry {
                        state: s2,
                        b1: if i == 1 { b[0] } else { x },
                        b2: if i == 1 { x } else { b[1] },
                    });
                    self.end += 1;
                }
            }
        }
        step
    }
}

const MOVE: [[i8; 5]; 13] = [
    [2, -1, 0, 0, 0],
    [2, 5, -1, 0, 0],
    [0, 1, 3, 6, -1],
    [2, 7, -1, 0, 0],
    [5, -1, 0, 0, 0],
    [1, 4, 6, 9, -1],
    [2, 5, 7, 10, -1],
    [3, 6, 8, 11, -1],
    [7, -1, 0, 0, 0],
    [5, 10, -1, 0, 0],
    [6, 9, 11, 12, -1],
    [7, 10, -1, 0, 0],
    [10, -1, 0, 0, 0],
];

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut solver = PuzzleSolver::new();
    let initial_state: UL = 0x123456789ab0;

    solver.insert(initial_state, 0);
    solver.sach(initial_state, 0, 12, 0, 1, 16);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut state: UL = 0;
        let mut b1 = -1;
        let mut b2 = -1;

        for (i, p) in line.split_whitespace().enumerate() {
            let p: i32 = p.parse().unwrap();
            if i == 0 && p < 0 {
                return;
            }
            if p == 0 {
                if b1 < 0 {
                    b1 = i as i8;
                } else {
                    b2 = i as i8;
                }
            }
            state = (state << 4) | (p as UL);
        }

        let ans = solver.lookup(state).unwrap_or(-1);
        let result = if ans < 0 {
            let computed_ans = solver.sach(state, b1, b2, 1, 1, 4);
            if computed_ans > 4 {
                -1
            } else {
                computed_ans + 16
            }
        } else {
            ans
        };

        if result < 0 {
            println!("NA");
        } else {
            println!("{}", result);
        }
    }
}
