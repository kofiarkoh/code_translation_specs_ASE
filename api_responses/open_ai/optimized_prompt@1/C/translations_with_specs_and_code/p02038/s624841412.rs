use std::io::{self, Write};

fn souti(a: char, b: char) -> char {
    if a == 'T' && b == 'F' {
        'F'
    } else {
        'T'
    }
}

fn main() -> Result<(), i32> {
    let mut input = String::new();
    
    // Read the number of boolean characters
    io::stdin().read_line(&mut input).map_err(|_| 1)?;
    let n: usize = input.trim().parse().map_err(|_| 1)?;

    // Preconditions check
    if n < 2 || n > 114514 {
        return Err(1);
    }

    let mut p = Vec::with_capacity(n);
    
    // Read the boolean characters
    for _ in 0..n {
        let mut hoge = String::new();
        io::stdin().read_line(&mut hoge).map_err(|_| 1)?;
        let ch = hoge.trim().chars().next().ok_or(1)?;

        // Invariant check
        if ch != 'T' && ch != 'F' {
            return Err(1);
        }
        p.push(ch);
    }

    // Apply the logical operation
    let mut notomamiko = souti(p[0], p[1]);
    for i in 2..n {
        notomamiko = souti(notomamiko, p[i]);
    }

    // Output the result
    println!("{}", notomamiko);
    Ok(())
}
