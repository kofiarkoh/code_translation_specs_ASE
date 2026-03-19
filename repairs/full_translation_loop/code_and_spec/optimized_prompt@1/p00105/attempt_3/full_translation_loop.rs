use std::io::{self, BufRead};

const MAX: usize = 100;
const PGMAX: usize = 1001;

#[derive(Debug)]
struct Index {
    pgnum: [i32; PGMAX],
    tm: i32,
    str: String,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut idx: Vec<Index> = Vec::new();
    let mut rgnum: usize = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut parts = line.split_whitespace();
        let tmp_str = parts.next().expect("Expected a string").to_string();
        let tmp_pgnum: usize = parts.next().expect("Expected an integer").parse().expect("Invalid integer");

        let mut flag = true;

        for i in 0..rgnum {
            if idx[i].str == tmp_str {
                idx[i].pgnum[tmp_pgnum] = 1;
                idx[i].tm += 1;
                flag = false; // Found existing string
                break;
            }
        }

        if flag {
            if rgnum < MAX {
                let mut new_entry = Index {
                    pgnum: [0; PGMAX],
                    tm: 1,
                    str: tmp_str,
                };
                new_entry.pgnum[tmp_pgnum] = 1;
                idx.push(new_entry);
                rgnum += 1;
            }
        }
    }

    // Sort the entries lexicographically by string
    idx.sort_by(|a, b| a.str.cmp(&b.str));

    // Output the results
    for entry in idx.iter() {
        println!("{}", entry.str);
        let mut k = 0;
        for j in 0..PGMAX {
            if entry.pgnum[j] == 1 {
                if k == entry.tm - 1 {
                    println!("{}", j);
                    break;
                } else {
                    print!("{} ", j);
                    k += 1;
                }
            }
        }
        if k > 0 {
            println!(); // Ensure a newline after the integers
        }
    }
}
