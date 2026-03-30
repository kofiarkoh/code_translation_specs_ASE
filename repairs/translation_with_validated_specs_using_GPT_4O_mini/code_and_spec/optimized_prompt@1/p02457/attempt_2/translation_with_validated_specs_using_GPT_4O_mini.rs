use std::io::{self, BufRead};

struct DynamicArray {
    elements: Vec<i32>,
}

impl DynamicArray {
    fn new() -> Self {
        DynamicArray {
            elements: Vec::new(),
        }
    }

    fn search(&self, key: i32) -> Option<usize> {
        let mut head = 0;
        let mut tail = self.elements.len() as isize - 1;

        while head <= tail {
            let mid = (head + tail) / 2;
            if self.elements[mid as usize] == key {
                return Some(mid as usize);
            } else if self.elements[mid as usize] > key {
                tail = mid - 1;
            } else {
                head = mid + 1;
            }
        }
        None
    }

    fn get_lower_bound(&self, key: i32) -> usize {
        let mut head = 0;
        let mut tail = self.elements.len() as isize - 1;

        if self.elements.is_empty() || self.elements[0] > key {
            return 0;
        }
        if self.elements[tail as usize] < key {
            return self.elements.len();
        }

        while head <= tail {
            let mid = (head + tail) / 2;
            if self.elements[mid as usize] < key {
                head = mid + 1;
            } else {
                if mid == 0 || self.elements[(mid - 1) as usize] < key {
                    return mid as usize;
                }
                tail = mid - 1;
            }
        }
        head as usize
    }

    fn insert(&mut self, key: i32) -> bool {
        if self.search(key).is_none() {
            let pos = self.get_lower_bound(key);
            self.elements.insert(pos, key);
            true
        } else {
            false
        }
    }

    fn delete(&mut self, key: i32) -> i32 {
        if let Some(index) = self.search(key) {
            self.elements.remove(index);
            1
        } else {
            -1
        }
    }

    fn dump(&self, l: i32, r: i32) {
        let start = self.get_lower_bound(l);
        let end = self.elements.iter().take_while(|&&x| x <= r).count();
        for &value in &self.elements[start..start + end] {
            println!("{}", value);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let q: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse number of operations");
    let mut dynamic_array = DynamicArray::new();

    for _ in 0..q {
        let command: i32 = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse command");
        match command {
            0 => {
                let x: i32 = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse insert value");
                if dynamic_array.insert(x) {
                    println!("{}", dynamic_array.elements.len());
                } else {
                    println!("{}", dynamic_array.elements.len());
                }
            }
            1 => {
                let x: i32 = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse find value");
                if dynamic_array.search(x).is_some() {
                    println!("1");
                } else {
                    println!("0");
                }
            }
            2 => {
                let x: i32 = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse delete value");
                dynamic_array.delete(x);
            }
            3 => {
                let (x, y): (i32, i32) = {
                    let line = lines.next().unwrap().unwrap();
                    let mut parts = line.split_whitespace();
                    let x = parts.next().unwrap().parse().expect("Failed to parse dump lower bound");
                    let y = parts.next().unwrap().parse().expect("Failed to parse dump upper bound");
                    (x, y)
                };
                dynamic_array.dump(x, y);
            }
            _ => {}
        }
    }
}
