use std::collections::HashMap;
use std::io::{self, BufRead};

const MAX: usize = 100;
const PGMAX: usize = 1001;
const WDMAX: usize = 31;

#[derive(Debug)]
struct Index {
    pgnum: [i32; PGMAX],
    tm: usize,
    str: String,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut idx: Vec<Index> = Vec::new();
    let mut rgnum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            continue; // Skip invalid input
        }

        let tmp_str = parts[0];
        let tmp_pgnum: i32 = parts[1].parse().unwrap_or(-1); // Handle invalid integers

        if tmp_pgnum < 0 || tmp_pgnum >= PGMAX as i32 {
            continue; // Skip out-of-range integers
        }

        let mut found = false;
        for i in 0..rgnum {
            if idx[i].str == tmp_str {
                idx[i].pgnum[tmp_pgnum as usize] = 1;
                idx[i].tm += 1;
                found = true;
                break;
            }
        }

        if !found {
            if rgnum < MAX {
                let mut new_entry = Index {
                    pgnum: [0; PGMAX],
                    tm: 1,
                    str: tmp_str.to_string(),
                };
                new_entry.pgnum[tmp_pgnum as usize] = 1;
                idx.push(new_entry);
                rgnum += 1;
            }
        }
    }

    // Sort the entries by string
    idx.sort_by(|a, b| a.str.cmp(&b.str));

    // Output the results
    for entry in idx.iter() {
        print!("{}\n", entry.str);
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
    }
}
