use std::io::{self, BufRead};

fn souti(a: char, b: char) -> char {
    if a == 'T' && b == 'F' {
        'F'
    } else {
        'T'
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0),
        _ => return Ok(()), // Return early if input is invalid
    };

    let mut p = vec![' '; 114514]; // Allocate space for P
    for i in 0..n {
        let hoge: String = match lines.next() {
            Some(Ok(line)) => line.trim().chars().take(3).collect(),
            _ => return Ok(()), // Return early if input is invalid
        };
        p[i] = hoge.chars().next().unwrap_or(' '); // Get the first character
    }

    let mut notomamiko = souti(p[0], p[1]);
    for i in 2..n {
        notomamiko = souti(notomamiko, p[i]);
    }

    println!("{}", notomamiko);
    Ok(())
}
