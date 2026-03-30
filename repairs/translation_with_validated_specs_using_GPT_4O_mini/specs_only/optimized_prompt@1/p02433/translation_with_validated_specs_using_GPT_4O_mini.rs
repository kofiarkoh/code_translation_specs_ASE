use std::collections::VecDeque;
use std::io;

struct Node {
    value: i32,
    prev: Option<Box<Node>>,
    next: Option<Box<Node>>,
}

struct DoublyLinkedList {
    head: Option<Box<Node>>,
    tail: Option<Box<Node>>,
}

impl DoublyLinkedList {
    fn new() -> Self {
        DoublyLinkedList { head: None, tail: None }
    }

    fn insert(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            prev: self.tail.take(),
            next: None,
        });

        if let Some(ref mut tail) = self.tail {
            tail.next = Some(new_node);
        } else {
            self.head = Some(new_node);
        }

        self.tail = self.head.as_mut().and_then(|node| node.next.as_mut());
    }

    fn erase(&mut self, value: i32) {
        let mut current = self.head.take();
        let mut prev: Option<Box<Node>> = None;

        while let Some(mut node) = current {
            if node.value == value {
                if let Some(ref mut prev_node) = prev {
                    prev_node.next = node.next.take();
                } else {
                    self.head = node.next.take();
                }

                if node.next.is_none() {
                    self.tail = prev;
                } else {
                    current = node.next.take();
                    current.as_mut().unwrap().prev = prev;
                }
                return;
            }
            prev = Some(node);
            current = prev.as_mut().unwrap().next.take();
        }
    }

    fn print(&self) {
        let mut current = self.head.as_ref();
        let mut output = VecDeque::new();

        while let Some(node) = current {
            output.push_back(node.value);
            current = node.next.as_ref();
        }

        // Print the contents excluding the sentinel value
        for value in output {
            print!("{} ", value);
        }
        println!();
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();
    let mut input = String::new();

    // Read number of operations
    io::stdin().read_line(&mut input).unwrap();
    let num_operations: usize = input.trim().parse().unwrap();

    for _ in 0..num_operations {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let command = parts[0];

        match command {
            "insert" => {
                let value: i32 = parts[1].parse().unwrap();
                list.insert(value);
            }
            "erase" => {
                let value: i32 = parts[1].parse().unwrap();
                list.erase(value);
            }
            _ => {
                eprintln!("Unknown command: {}", command);
                std::process::exit(1);
            }
        }
    }

    list.print();
}
