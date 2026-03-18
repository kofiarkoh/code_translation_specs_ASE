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

fn print_list(cursor: &Option<Box<Cell>>) {
    let mut current = cursor.as_ref();
    
    // Move to the front of the list
    while let Some(ref cell) = current {
        if cell.prev.is_none() {
            current = Some(cell);
            break;
        }
        current = cell.prev.as_ref();
    }

    // Print the list
    while let Some(cell) = current {
        if cell.data != END {
            println!("{}", cell.data);
        }
        current = cell.next.as_ref();
    }
}

fn is_end(cursor: &Option<Box<Cell>>) -> bool {
    cursor.as_ref().map_or(false, |c| c.data == END)
}

fn is_front(cursor: &Option<Box<Cell>>) -> bool {
    cursor.as_ref().map_or(false, |c| c.prev.is_none())
}

fn insert(cursor: Option<Box<Cell>>, data: i32) -> Option<Box<Cell>> {
    let mut new_cell = Box::new(Cell::new(data));
    
    if is_front(&cursor) {
        new_cell.next = cursor;
        return Some(new_cell);
    }

    let mut cursor = cursor?;
    new_cell.prev = cursor.prev.take();
    new_cell.next = Some(cursor);
    cursor.prev = Some(new_cell.clone());
    
    Some(new_cell)
}

fn move_cursor(cursor: Option<Box<Cell>>, d: i32) -> Option<Box<Cell>> {
    let mut cursor = cursor?;
    let mut steps = d;

    if steps < 0 {
        while steps < 0 {
            if is_front(&Some(cursor.clone())) {
                eprintln!("Cursor is already front.");
                std::process::exit(1);
            }
            cursor = cursor.prev.take()?;
            steps += 1;
        }
    } else {
        while steps > 0 {
            if cursor.next.is_none() {
                eprintln!("Cursor has no next.");
                std::process::exit(2);
            }
            cursor = cursor.next.take()?;
            steps -= 1;
        }
    }
    
    Some(cursor)
}

fn erase(cursor: Option<Box<Cell>>) -> Option<Box<Cell>> {
    if is_end(&cursor) {
        return cursor;
    }

    let mut cursor = cursor?;
    if is_front(&Some(cursor.clone())) {
        let next = cursor.next.take();
        if let Some(ref mut next_cell) = next {
            next_cell.prev = None;
        }
        return next;
    }

    let prev = cursor.prev.take();
    let next = cursor.next.take();
    
    if let Some(ref mut prev_cell) = prev {
        prev_cell.next = next.clone();
    }
    if let Some(ref mut next_cell) = next {
        next_cell.prev = prev;
    }
    
    Some(next.unwrap_or(cursor))
}

fn answer() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let q: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut cursor = Some(Box::new(Cell::new(END)));

    for _ in 0..q {
        let command = lines.next().unwrap().unwrap();
        let command_type = command.chars().next().unwrap();
        let value: i32 = command[2..].trim().parse().unwrap();

        match command_type {
            INSERT => cursor = insert(cursor, value),
            MOVE => cursor = move_cursor(cursor, value),
            ERASE => cursor = erase(cursor),
            _ => {}
        }
    }

    print_list(&cursor);
}

fn main() {
    answer();
}
