use std::io::{self, BufRead};

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
    cursor: Option<*mut Node>, // Using raw pointer for cursor to mimic pointer behavior
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
        self.cursor = self.head.as_deref_mut().map(|node| &mut **node as *mut _); // Move cursor to new head
    }

    fn move_cursor(&mut self, direction: &str) {
        match direction {
            "next" => {
                if let Some(current) = self.cursor {
                    unsafe {
                        if let Some(ref mut node) = (*current).next {
                            self.cursor = Some(&mut **node as *mut _);
                        } else {
                            eprintln!("Error: Attempted to move cursor beyond the end of the list.");
                            std::process::exit(1);
                        }
                    }
                } else {
                    eprintln!("Error: Cursor is not pointing to a valid node.");
                    std::process::exit(1);
                }
            }
            "prev" => {
                // Moving to previous is not implemented as per the specification
                eprintln!("Error: Moving cursor to previous is not supported.");
                std::process::exit(1);
            }
            _ => {
                eprintln!("Error: Invalid move command.");
                std::process::exit(1);
            }
        }
    }

    fn erase(&mut self) {
        if let Some(current) = self.cursor {
            unsafe {
                if let Some(ref mut node) = (*current).next {
                    // Erase current node by skipping it
                    self.cursor = Some(&mut **node as *mut _);
                } else {
                    // Erasing the last node has no effect
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
    let mut commands = handle.lines().filter_map(Result::ok);

    let command_count: usize = commands.next().unwrap().parse().unwrap();
    let mut list = LinkedList::new();

    for _ in 0..command_count {
        if let Some(command) = commands.next() {
            let parts: Vec<&str> = command.split_whitespace().collect();
            match parts.as_slice() {
                ["insert", value] => {
                    let value: i32 = value.parse().unwrap();
                    list.insert(value);
                }
                ["move", direction] => {
                    list.move_cursor(direction);
                }
                ["erase"] => {
                    list.erase();
                }
                _ => {
                    eprintln!("Error: Invalid command.");
                    std::process::exit(1);
                }
            }
        }
    }

    list.print();
}
