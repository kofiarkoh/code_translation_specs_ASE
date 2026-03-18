use std::io::{self, BufRead};

const INSERT: char = '0';
const MOVE: char = '1';
const ERASE: char = '2';
const END: i32 = 1_000_000_001;

#[derive(Debug)]
struct Cell {
    data: i32,
    prev: Option<Box<Cell>>,
    next: Option<Box<Cell>>,
}

impl Cell {
    fn new(data: i32) -> Self {
        Cell {
            data,
            prev: None,
            next: None,
        }
    }
}

fn print_list(cursor: &Cell) {
    let mut current = cursor;

    // Move to the front of the list
    while let Some(ref prev) = current.prev {
        current = prev;
    }

    // Print the list until the sentinel value
    while current.data != END {
        println!("{}", current.data);
        if let Some(ref next) = current.next {
            current = next;
        } else {
            break;
        }
    }
}

fn is_end(cursor: &Cell) -> bool {
    cursor.data == END
}

fn is_front(cursor: &Cell) -> bool {
    cursor.prev.is_none()
}

fn insert(cursor: &Cell, data: i32) -> Cell {
    let new_cell = Cell::new(data);
    if is_front(cursor) {
        let mut new_cell = Box::new(new_cell);
        new_cell.next = Some(Box::new(cursor.clone()));
        return *new_cell;
    }

    let mut new_cell = Box::new(new_cell);
    if let Some(ref mut prev) = cursor.prev {
        new_cell.prev = Some(prev.clone());
        new_cell.next = Some(Box::new(cursor.clone()));
        cursor.prev = Some(new_cell.clone());
        prev.next = Some(new_cell);
    }
    cursor.clone()
}

fn move_cursor(cursor: &Cell, d: i32) -> Cell {
    let mut current = cursor.clone();
    if d < 0 {
        for _ in 0..-d {
            if is_front(&current) {
                eprintln!("Cursor is already front.");
                std::process::exit(1);
            }
            if let Some(ref prev) = current.prev {
                current = prev.clone();
            }
        }
    } else {
        for _ in 0..d {
            if current.next.is_none() {
                eprintln!("Cursor has no next.");
                std::process::exit(2);
            }
            if let Some(ref next) = current.next {
                current = next.clone();
            }
        }
    }
    current
}

fn erase(cursor: &Cell) -> Cell {
    if is_end(cursor) {
        return cursor.clone();
    }

    if is_front(cursor) {
        if let Some(ref next) = cursor.next {
            next.prev = None;
            return next.clone();
        }
    } else {
        if let Some(ref prev) = cursor.prev {
            if let Some(ref next) = cursor.next {
                prev.next = Some(next.clone());
                next.prev = Some(prev.clone());
            }
        }
    }
    cursor.clone()
}

fn answer() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let q: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut cursor = Cell::new(END);

    for _ in 0..q {
        let command = lines.next().unwrap().unwrap();
        let command_type = command.chars().next().unwrap();
        let value: i32 = command[2..].trim().parse().unwrap();

        match command_type {
            INSERT => cursor = insert(&cursor, value),
            MOVE => cursor = move_cursor(&cursor, value),
            ERASE => cursor = erase(&cursor),
            _ => {}
        }
    }

    print_list(&cursor);
}

fn main() {
    answer();
}
