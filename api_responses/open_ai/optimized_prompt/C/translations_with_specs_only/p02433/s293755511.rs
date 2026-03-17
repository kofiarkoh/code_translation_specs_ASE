use std::io::{self, BufRead};

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
    cursor: Option<*mut Node>, // Using raw pointer for cursor to emulate pointer-like behavior
}

impl LinkedList {
    fn new() -> Self {
        LinkedList {
            head: None,
            cursor: None,
        }
    }

    fn insert(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.cursor = self.head.as_deref_mut().map(|node| node.as_mut() as *mut _);
    }

    fn move_cursor(&mut self, direction: i32) {
        if direction < 0 {
            // Move cursor to the previous node
            if let Some(cursor) = self.cursor {
                let mut current = self.head.as_deref_mut();
                let mut prev: *mut Node = std::ptr::null_mut();
                while let Some(node) = current {
                    if node as *mut _ == cursor {
                        if prev.is_null() {
                            // Attempting to move before the head
                            eprintln!("Error: Cannot move cursor before the head of the list.");
                            std::process::exit(1);
                        }
                        self.cursor = Some(prev);
                        return;
                    }
                    prev = node as *mut _;
                    current = node.next.as_deref_mut();
                }
            }
        } else if direction > 0 {
            // Move cursor to the next node
            if let Some(cursor) = self.cursor {
                let mut current = self.head.as_deref_mut();
                while let Some(node) = current {
                    if node as *mut _ == cursor {
                        if let Some(next) = node.next.as_deref_mut() {
                            self.cursor = Some(next as *mut _);
                        } else {
                            // Attempting to move beyond the end of the list
                            eprintln!("Error: Cannot move cursor beyond the end of the list.");
                            std::process::exit(1);
                        }
                        return;
                    }
                    current = node.next.as_deref_mut();
                }
            }
        }
    }

    fn erase(&mut self) {
        if let Some(cursor) = self.cursor {
            if let Some(mut current) = self.head.take() {
                if current.as_mut() as *mut _ == cursor {
                    // Erasing the head
                    self.head = current.next.take();
                    self.cursor = self.head.as_deref_mut().map(|node| node as *mut _);
                    return;
                }
                let mut prev: Option<Box<Node>> = None;
                while let Some(next) = current.next.take() {
                    if next.as_mut() as *mut _ == cursor {
                        // Erasing the current node
                        current.next = next.next.take();
                        self.cursor = current.next.as_deref_mut().map(|node| node as *mut _);
                        return;
                    }
                    prev = Some(current);
                    current = next;
                }
            }
        }
    }

    fn print(&self) {
        let mut current = self.head.as_deref();
        while let Some(node) = current {
            println!("{}", node.value);
            current = node.next.as_deref();
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let command_count: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut list = LinkedList::new();

    for _ in 0..command_count {
        if let Some(command) = lines.next() {
            let command = command.unwrap();
            let parts: Vec<&str> = command.split_whitespace().collect();
            match parts[0] {
                "insert" => {
                    if let Ok(value) = parts[1].parse::<i32>() {
                        list.insert(value);
                    }
                }
                "move" => {
                    if let Ok(direction) = parts[1].parse::<i32>() {
                        list.move_cursor(direction);
                    }
                }
                "erase" => {
                    list.erase();
                }
                _ => {}
            }
        }
    }

    list.print();
}
