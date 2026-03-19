use std::io::{self, BufRead};

const FH: usize = 0; // First half
const SH: usize = 1; // Second half
const ALL: usize = 2;
const MAX: usize = 200005;

struct Queue {
    que: [[i32; MAX]; 2],
    qt: [usize; 2],
    qe: [usize; 2],
    min: [[i32; MAX]; 2],
    at: [usize; 2],
    ae: [usize; 2],
    max: [[i32; MAX]; 2],
    zt: [usize; 2],
    ze: [usize; 2],
}

impl Queue {
    fn new() -> Self {
        Queue {
            que: [[0; MAX]; 2],
            qt: [0; 2],
            qe: [0; 2],
            min: [[0; MAX]; 2],
            at: [0; 2],
            ae: [0; 2],
            max: [[0; MAX]; 2],
            zt: [0; 2],
            ze: [0; 2],
        }
    }

    fn size(&self, id: usize) -> usize {
        self.qe[id] - self.qt[id]
    }

    fn min_sz(&self, id: usize) -> usize {
        self.ae[id] - self.at[id]
    }

    fn max_sz(&self, id: usize) -> usize {
        self.ze[id] - self.zt[id]
    }

    fn del(&mut self, id: usize) -> i32 {
        let a;
        if id == ALL {
            a = self.del(FH);
            if self.size(FH) < self.size(SH) {
                self.add(FH, self.del(SH));
            }
        } else if id == FH {
            a = self.que[id][self.qe[id] - 1];
            self.qe[id] -= 1;
            if self.min[id][self.ae[id] - 1] == a {
                self.ae[id] -= 1;
            }
            if self.max[id][self.ze[id] - 1] == a {
                self.ze[id] -= 1;
            }
        } else {
            a = self.que[id][self.qt[id]];
            self.qt[id] += 1;
            if self.min[id][self.at[id]] == a {
                self.at[id] += 1;
            }
            if self.max[id][self.zt[id]] == a {
                self.zt[id] += 1;
            }
        }
        a
    }

    fn add(&mut self, id: usize, a: i32) {
        if id == ALL {
            self.add(SH, a);
            if self.size(FH) < self.size(SH) {
                self.add(FH, self.del(SH));
            }
            return;
        }
        self.que[id][self.qe[id]] = a;
        self.qe[id] += 1;
        if self.size(id) == 1 {
            self.min[id][self.ae[id]] = a;
            self.max[id][self.ze[id]] = a;
            self.ae[id] += 1;
            self.ze[id] += 1;
        } else if id == FH {
            if self.min[id][self.ae[id] - 1] > a {
                self.min[id][self.ae[id]] = a;
                self.ae[id] += 1;
            }
            if self.max[id][self.ze[id] - 1] < a {
                self.max[id][self.ze[id]] = a;
                self.ze[id] += 1;
            }
        } else {
            while self.min_sz(id) > 0 && self.min[id][self.ae[id] - 1] > a {
                self.ae[id] -= 1;
            }
            while self.max_sz(id) > 0 && self.max[id][self.ze[id] - 1] < a {
                self.ze[id] -= 1;
            }
            self.min[id][self.ae[id]] = a;
            self.max[id][self.ze[id]] = a;
            self.ae[id] += 1;
            self.ze[id] += 1;
        }
    }

    fn gmin(&self, id: usize, th: usize) -> i32 {
        if id == FH {
            self.min[id][self.ae[id] - th]
        } else {
            self.min[id][self.at[id] + th - 1]
        }
    }

    fn gmax(&self, id: usize, th: usize) -> i32 {
        if id == FH {
            self.max[id][self.ze[id] - th]
        } else {
            self.max[id][self.zt[id] + th - 1]
        }
    }
}

fn in_value() -> i32 {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = handle.lines().filter_map(Result::ok);
    let mut n = 0;
    let mut c = input.next().unwrap().chars().next().unwrap();
    while c < '0' {
        c = input.next().unwrap().chars().next().unwrap();
    }
    while c >= '0' {
        n = 10 * n + (c as i32 - '0' as i32);
        c = input.next().unwrap().chars().next().unwrap();
    }
    n
}

fn main() {
    let mut queue = Queue::new();
    let mut input = io::stdin().lock().lines().filter_map(Result::ok);

    while let Some(line) = input.next() {
        let q: i32 = line.trim().parse().unwrap();
        queue.qt = [0, 0];
        queue.qe = [0, 0];
        queue.at = [0, 0];
        queue.ae = [0, 0];
        queue.zt = [0, 0];
        queue.ze = [0, 0];

        for _ in 0..q {
            let op: i32 = in_value();
            let a: i32;
            match op {
                0 => {
                    queue.add(ALL, in_value());
                    continue;
                }
                1 => a = queue.del(ALL),
                2 => a = queue.gmin(FH, 1),
                3 => a = queue.gmin(SH, 1),
                4 => a = queue.gmin(FH, in_value()),
                5 => a = queue.gmin(SH, in_value()),
                6 => a = queue.gmax(FH, 1),
                7 => a = queue.gmax(SH, 1),
                8 => a = queue.gmax(FH, in_value()),
                _ => a = queue.gmax(SH, in_value()),
            }
            println!("{}", a);
        }
        println!("end");
    }
}
