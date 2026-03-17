use std::io::{self, BufRead};

const MAX: usize = 100;
const PGMAX: usize = 1001;
const WDMAX: usize = 31;

#[derive(Default)]
struct Index {
    pgnum: [i32; PGMAX],
    tm: i32,
    str: [char; WDMAX],
}

impl Index {
    fn new() -> Self {
        Index {
            pgnum: [0; PGMAX],
            tm: 0,
            str: ['\0'; WDMAX],
        }
    }

    fn from_str(s: &str) -> Self {
        let mut idx = Index::new();
        idx.tm = 1; // Initialize tm to 1 for the first occurrence
        let chars: Vec<char> = s.chars().collect();
        for (i, &c) in chars.iter().enumerate() {
            if i < WDMAX {
                idx.str[i] = c;
            } else {
                break; // Prevent overflow
            }
        }
        idx
    }
}

fn swap(i: usize, j: usize, idx: &mut [Index; MAX]) {
    let tmp_str = idx[i].str;
    let tmp_pgnum = idx[i].pgnum;
    let tmp_tm = idx[i].tm;

    idx[i].str = idx[j].str;
    idx[i].pgnum = idx[j].pgnum;
    idx[i].tm = idx[j].tm;

    idx[j].str = tmp_str;
    idx[j].pgnum = tmp_pgnum;
    idx[j].tm = tmp_tm;
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut idx: [Index; MAX] = Default::default();
    let mut rgnum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue; // Skip invalid lines
        }
        let tmp_str = parts[0];
        let tmp_pgnum: usize = parts[1].parse().unwrap_or(0);

        let mut flag = true;
        for i in 0..rgnum {
            if tmp_str == idx[i].str.iter().collect::<String>().trim_end_matches('\0') {
                idx[i].pgnum[tmp_pgnum] = 1;
                idx[i].tm += 1;
                flag = false;
                break;
            }
        }

        if flag {
            idx[rgnum] = Index::from_str(tmp_str);
            idx[rgnum].pgnum[tmp_pgnum] = 1;
            rgnum += 1;
        }
    }

    for i in 0..rgnum {
        for j in (i + 1)..rgnum {
            if idx[i].str.iter().collect::<String>() > idx[j].str.iter().collect::<String>() {
                swap(i, j, &mut idx);
            }
        }
    }

    for i in 0..rgnum {
        let str_output: String = idx[i].str.iter().collect();
        println!("{}", str_output.trim_end_matches('\0'));
        let mut k = 0;
        for j in 0..PGMAX {
            if idx[i].pgnum[j] == 1 {
                if k == idx[i].tm - 1 {
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
