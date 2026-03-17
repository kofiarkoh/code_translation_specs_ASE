use std::io::{self, Write};

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
    cursor: *mut Node, // Using a raw pointer for cursor to mimic pointer behavior
}

impl LinkedList {
    fn new() -> Self {
        LinkedList {
            head: None,
            cursor: std::ptr::null_mut(), // Initialize cursor to null
        }
    }

    fn insert(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.cursor = self.head.as_deref_mut().map_or(std::ptr::null_mut(), |node| node.as_mut());
    }

    fn move_cursor(&mut self, direction: i32) {
        if direction == 1 {
            // Move forward
            if let Some(ref mut node) = self.head {
                self.cursor = node.as_mut() as *mut _; // Move cursor to the head
            }
        } else if direction == -1 {
            // Move backward (not implemented, just for demonstration)
            // In a real linked list, we would need a doubly linked list to move backward
            eprintln!("Error: Cannot move backward from the head.");
            std::process::exit(1);
        }
    }

    fn erase(&mut self) {
        if self.cursor.is_null() {
            eprintln!("Error: Cannot erase from an empty list.");
            std::process::exit(1);
        }
        // Erase logic would go here (not implemented for simplicity)
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
    let mut list = LinkedList::new();
    let commands = vec!["0 5", "1 1", "2"]; // Example commands

    for command in commands {
        let parts: Vec<&str> = command.split_whitespace().collect();
        match parts[0] {
            "0" => {
                if let Ok(value) = parts[1].parse::<i32>() {
                    list.insert(value);
                }
            }
            "1" => {
                if let Ok(direction) = parts[1].parse::<i32>() {
                    list.move_cursor(direction);
                }
            }
            "2" => {
                list.erase();
            }
            _ => {
                eprintln!("Error: Invalid command.");
                std::process::exit(1);
            }
        }
    }

    list.print();
}
