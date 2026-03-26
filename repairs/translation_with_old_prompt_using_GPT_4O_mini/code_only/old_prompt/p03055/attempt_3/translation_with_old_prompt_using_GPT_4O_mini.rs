use std::io::{self, BufRead};

const MAX_SIZE: usize = 400005;

struct Heap {
    h: [usize; MAX_SIZE],
    l: usize,
}

impl Heap {
    fn new() -> Self {
        Heap { h: [0; MAX_SIZE], l: 0 }
    }

    fn comp_h(&self, x: usize, y: usize) -> i32 {
        unsafe {
            if a[self.h[x]] > a[self.h[y]] {
                1
            } else {
                -1
            }
        }
    }

    fn swap_h(&mut self, x: usize, y: usize) {
        let f = self.h[x];
        self.h[x] = self.h[y];
        self.h[y] = f;
    }

    fn push(&mut self, ne: usize) {
        self.h[self.l] = ne;
        let mut p = self.l;
        self.l += 1;
        while p > 0 {
            let parent = (p - 1) / 2;
            if self.comp_h(parent, p) > 0 {
                self.swap_h(parent, p);
                p = parent;
            } else {
                break;
            }
        }
    }

    fn pop(&mut self) -> usize {
        self.l -= 1;
        self.swap_h(0, self.l);
        let mut p = 0;
        loop {
            if 2 * p + 2 < self.l {
                if self.comp_h(2 * p + 1, 2 * p + 2) > 0 {
                    if self.comp_h(p, 2 * p + 2) > 0 {
                        self.swap_h(p, 2 * p + 2);
                        p = 2 * p + 2;
                    } else {
                        break;
                    }
                } else {
                    if self.comp_h(p, 2 * p + 1) > 0 {
                        self.swap_h(p, 2 * p + 1);
                        p = 2 * p + 1;
                    } else {
                        break;
                    }
                }
            } else if 2 * p + 1 < self.l {
                if self.comp_h(p, 2 * p + 1) > 0 {
                    self.swap_h(p, 2 * p + 1);
                    p = 2 * p + 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        self.h[self.l]
    }
}

static mut a: [i32; MAX_SIZE] = [0; MAX_SIZE];
static mut b: [i32; MAX_SIZE] = [0; MAX_SIZE];

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();
    
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut i = 0;

    unsafe {
        for _ in 0..n - 1 {
            i *= 2;
            let line = lines.next().unwrap().unwrap();
            let nums: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            a[i] = nums[0] - 1;
            b[i] = nums[1] - 1;
            a[i + 1] = b[i];
            b[i + 1] = a[i];
            i /= 2;
        }
    }

    let mut heap = Heap::new();
    for i in 0..2 * n - 2 {
        heap.push(i);
    }

    let mut c = [0; MAX_SIZE];
    for i in 0..2 * n - 2 {
        c[i] = heap.pop();
    }
    c[2 * n - 2] = 2 * n - 2;
    unsafe { a[2 * n - 2] = -1; }

    let mut depth = [-1; 200005];
    let mut q = [0; 200005];
    depth[0] = 1;
    q[0] = 0;
    let mut l = 1;

    while l > 0 {
        l -= 1;
        let i = q[l];
        let mut min: usize = usize::MAX; // Changed to usize to avoid negative values
        let mut max = 2 * n - 2;

        while max > min + 1 {
            let mid = (max + min) / 2;
            unsafe {
                if a[c[mid]] < i {
                    min = mid;
                } else {
                    max = mid;
                }
            }
        }

        while unsafe { a[c[max]] } == i {
            if depth[unsafe { b[c[max]] } as usize] < 0 {
                depth[unsafe { b[c[max]] } as usize] = depth[i] + 1;
                q[l] = unsafe { b[c[max]] } as usize; // Ensure index is usize
                l += 1;
            }
            max += 1;
        }
    }

    let mut s = 0;
    for i in 0..n {
        if depth[s] < depth[i] {
            s = i;
        }
    }

    for i in 0..n {
        depth[i] = -1;
    }
    depth[s] = 1;
    q[0] = s as usize; // Ensure index is usize
    l = 1;

    while l > 0 {
        l -= 1;
        let i = q[l];
        let mut min: usize = usize::MAX; // Changed to usize to avoid negative values
        let mut max = 2 * n - 2;

        while max > min + 1 {
            let mid = (max + min) / 2;
            unsafe {
                if a[c[mid]] < i {
                    min = mid;
                } else {
                    max = mid;
                }
            }
        }

        while unsafe { a[c[max]] } == i {
            if depth[unsafe { b[c[max]] } as usize] < 0 {
                depth[unsafe { b[c[max]] } as usize] = depth[i] + 1;
                q[l] = unsafe { b[c[max]] } as usize; // Ensure index is usize
                l += 1;
            }
            max += 1;
        }
    }

    let mut max = 0;
    for i in 0..n {
        if max < depth[i] {
            max = depth[i];
        }
    }

    if (max + 1) % 3 > 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
