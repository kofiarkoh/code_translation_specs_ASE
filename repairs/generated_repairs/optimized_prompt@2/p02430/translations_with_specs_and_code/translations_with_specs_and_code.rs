use std::io::{self, BufRead};

fn get_ints() -> Result<(usize, usize), io::Error> {
    let stdin = io::stdin();
    let mut handle = stdin.lock(); // Changed to mutable
    let mut line = String::new();
    handle.read_line(&mut line)?;
    let mut nums = line.split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok());
    let n = nums.next().ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Missing n"))?;
    let k = nums.next().ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Missing k"))?;
    Ok((n, k))
}

fn main() -> Result<(), io::Error> {
    let (n, k) = get_ints()?;

    // Check preconditions
    if k > n {
        return Ok(()); // No output if k > n
    }

    for pat in 0..(1 << n) {
        if pat.count_ones() as usize != k {
            continue;
        }
        print!("{}:", pat);
        let indices: Vec<usize> = (0..n)
            .filter(|i| pat & (1 << i) != 0)
            .collect();
        for index in indices {
            print!(" {}", index);
        }
        println!(); // Newline after each combination
    }
    Ok(())
}
