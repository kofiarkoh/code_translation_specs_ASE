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

    fn min_size(&self, id: usize) -> usize {
        self.ae[id] - self.at[id]
    }

    fn max_size(&self, id: usize) -> usize {
        self.ze[id] - self.zt[id]
    }

    fn del(&mut self, id: usize) -> Option<i32> {
        if id == ALL {
            let a = self.del(FH)?;
            if self.size(FH) < self.size(SH) {
                if let Some(value) = self.del(SH) {
                    self.add(FH, value);
                }
            }
            Some(a)
        } else if id == FH {
            let a = self.que[id][self.qe[id] - 1];
            self.qe[id] -= 1;
            if self.min[id][self.ae[id] - 1] == a {
                self.ae[id] -= 1;
            }
            if self.max[id][self.ze[id] - 1] == a {
                self.ze[id] -= 1;
            }
            Some(a)
        } else {
            let a = self.que[id][self.qt[id]];
            self.qt[id] += 1;
            if self.min[id][self.at[id]] == a {
                self.at[id] += 1;
            }
            if self.max[id][self.zt[id]] == a {
                self.zt[id] += 1;
            }
            Some(a)
        }
    }

    fn add(&mut self, id: usize, a: i32) {
        if id == ALL {
            self.add(SH, a);
            if self.size(FH) < self.size(SH) {
                if let Some(value) = self.del(SH) {
                    self.add(FH, value);
                }
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
            while self.min_size(id) > 0 && self.min[id][self.ae[id] - 1] > a {
                self.ae[id] -= 1;
            }
            while self.max_size(id) > 0 && self.max[id][self.ze[id] - 1] < a {
                self.ze[id] -= 1;
            }
            self.min[id][self.ae[id]] = a;
            self.max[id][self.ze[id]] = a;
            self.ae[id] += 1;
            self.ze[id] += 1;
        }
    }

    fn gmin(&self, id: usize, th: usize) -> Option<i32> {
        if id == FH {
            Some(self.min[id][self.ae[id] - th])
        } else {
            Some(self.min[id][self.at[id] + th - 1])
        }
    }

    fn gmax(&self, id: usize, th: usize) -> Option<i32> {
        if id == FH {
            Some(self.max[id][self.ze[id] - th])
        } else {
            Some(self.max[id][self.zt[id] + th - 1])
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().filter_map(Result::ok);
    
    let mut queue = Queue::new();

    while let Some(line) = lines.next() {
        let q: usize = line.trim().parse().expect("Failed to parse number of operations");
        queue.qt = [0; 2];
        queue.qe = [0; 2];
        queue.at = [0; 2];
        queue.ae = [0; 2];
        queue.zt = [0; 2];
        queue.ze = [0; 2];

        for _ in 0..q {
            let op: usize = lines.next().unwrap().trim().parse().expect("Failed to parse operation code");
            let a: Option<i32>;

            match op {
                0 => {
                    let value: i32 = lines.next().unwrap().trim().parse().expect("Failed to parse value for add operation");
                    queue.add(ALL, value);
                    continue;
                }
                1 => a = queue.del(ALL),
                2 => a = queue.gmin(FH, 1),
                3 => a = queue.gmin(SH, 1),
                4 => {
                    let th: usize = lines.next().unwrap().trim().parse().expect("Failed to parse threshold for gmin");
                    a = queue.gmin(FH, th);
                }
                5 => {
                    let th: usize = lines.next().unwrap().trim().parse().expect("Failed to parse threshold for gmin");
                    a = queue.gmin(SH, th);
                }
                6 => a = queue.gmax(FH, 1),
                7 => a = queue.gmax(SH, 1),
                8 => {
                    let th: usize = lines.next().unwrap().trim().parse().expect("Failed to parse threshold for gmax");
                    a = queue.gmax(FH, th);
                }
                _ => {
                    let th: usize = lines.next().unwrap().trim().parse().expect("Failed to parse threshold for gmax");
                    a = queue.gmax(SH, th);
                }
            }

            if let Some(value) = a {
                println!("{}", value);
            }
        }
        println!("end");
    }
}
