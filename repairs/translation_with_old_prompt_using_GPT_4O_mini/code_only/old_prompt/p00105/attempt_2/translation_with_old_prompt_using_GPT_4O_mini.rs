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
        let mut parts = line.split_whitespace();
        let tmp_str: String = parts.next().unwrap().to_string();
        let tmp_pgnum: usize = parts.next().unwrap().parse().unwrap();

        let mut flag = true;
        for i in 0..rgnum {
            if idx[i].str.iter().collect::<String>() == tmp_str {
                idx[i].pgnum[tmp_pgnum] = 1;
                idx[i].tm += 1;
                flag = false;
                break;
            }
        }

        if flag {
            idx[rgnum].pgnum[tmp_pgnum] = 1;
            idx[rgnum].tm += 1;
            idx[rgnum].str[..tmp_str.len()].copy_from_slice(&tmp_str.chars().collect::<Vec<_>>()[..]);
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
        let str_output: String = idx[i].str.iter().take_while(|&&c| c != '\0').collect();
        println!("{}", str_output);
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
        if k > 0 {
            println!(); // Ensure a newline after the last printed page numbers
        }
    }
}
