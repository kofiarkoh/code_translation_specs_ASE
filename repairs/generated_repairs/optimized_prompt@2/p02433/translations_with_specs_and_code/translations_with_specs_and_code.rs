use std::io::{self, BufRead};

const INSERT: char = '0';
const MOVE: char = '1';
const ERASE: char = '2';
const END: i32 = 1_000_000_001;

#[derive(Debug, Clone)]
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
    let mut current = Some(current);
    while let Some(ref cell) = current {
        if cell.data != END {
            println!("{}", cell.data);
        }
        current = cell.next.as_deref();
    }
}

fn is_end(cursor: &Option<Box<Cell>>) -> bool {
    cursor.as_ref().map_or(false, |c| c.data == END)
}

fn is_front(cursor: &Option<Box<Cell>>) -> bool {
    cursor.as_ref().map_or(false, |c| c.prev.is_none())
}

fn insert(cursor: &mut Option<Box<Cell>>, data: i32) -> Option<Box<Cell>> {
    let new_cell = Box::new(Cell::new(data));

    if is_front(cursor) {
        let mut new_cell = new_cell;
        new_cell.next = cursor.take();
        if let Some(ref mut next) = new_cell.next {
            next.prev = Some(new_cell.clone());
        }
        return Some(new_cell);
    }

    let mut new_cell = new_cell;
    if let Some(ref mut current) = cursor {
        let prev = current.prev.take();
        new_cell.prev = prev;
        new_cell.next = Some(current.clone());
        current.prev = Some(Box::new(new_cell.clone()));
        if let Some(ref mut prev_cell) = new_cell.prev {
            prev_cell.next = Some(new_cell);
        }
    }
    Some(new_cell)
}

fn move_cursor(cursor: &mut Option<Box<Cell>>, d: i32) -> Option<Box<Cell>> {
    if d == 0 {
        return cursor.clone();
    }

    let mut cursor = cursor;

    if d < 0 {
        for _ in 0..-d {
            if is_front(cursor) {
                eprintln!("Cursor is already front.");
                std::process::exit(1);
            }
            if let Some(ref mut current) = cursor {
                cursor = current.prev.as_deref_mut();
            }
        }
        return cursor.clone();
    }

    for _ in 0..d {
        if let Some(ref current) = cursor {
            if current.next.is_none() {
                eprintln!("Cursor has no next.");
                std::process::exit(2);
            }
            cursor = current.next.as_deref_mut();
        }
    }

    cursor.clone()
}

fn erase(cursor: &mut Option<Box<Cell>>) -> Option<Box<Cell>> {
    if is_end(cursor) {
        return cursor.clone();
    }

    if is_front(cursor) {
        if let Some(ref mut current) = cursor {
            let next = current.next.take();
            if let Some(ref mut next_cell) = next {
                next_cell.prev = None;
            }
            return next;
        }
    } else {
        if let Some(ref mut current) = cursor {
            let prev = current.prev.take();
            let next = current.next.take();
            if let Some(ref mut prev_cell) = prev {
                prev_cell.next = next.clone();
            }
            if let Some(ref mut next_cell) = next {
                next_cell.prev = prev;
            }
            return next;
        }
    }
    cursor.clone()
}

fn answer() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let q: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut cursor = Some(Box::new(Cell::new(END)));

    for _ in 0..q {
        let command = lines.next().unwrap().unwrap();
        let command_type = command.chars().next().unwrap();
        let value: i32 = command[2..].trim().parse().unwrap();

        match command_type {
            INSERT => cursor = insert(&mut cursor, value),
            MOVE => cursor = move_cursor(&mut cursor, value),
            ERASE => cursor = erase(&mut cursor),
            _ => {}
        }
    }

    if let Some(ref cell) = cursor {
        print_list(cell);
    }
}

fn main() {
    answer();
}
