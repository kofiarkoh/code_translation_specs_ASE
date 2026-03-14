use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut word: Vec<String> = Vec::new();
    let mut num: Vec<i32> = Vec::new();
    let mut dict: Vec<String> = Vec::new();
    let mut mark: Vec<i32> = vec![0; 128];
    let mut page: Vec<Vec<i32>> = vec![vec![0; 128]; 128];
    
    let mut input = stdin.lock().lines();
    let mut i = 0;

    while let Some(Ok(line)) = input.next() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue; // Skip invalid lines
        }
        word.push(parts[0].to_string());
        num.push(parts[1].parse::<i32>().unwrap());

        if i > 0 {
            let mut j = 0;
            while j < i {
                if j + 1 < i && word[j] > word[j + 1] {
                    word.swap(j, j + 1);
                    num.swap(j, j + 1);
                    j = 0; // Restart sorting
                } else {
                    j += 1;
                }
            }
        }
        i += 1;
    }

    let mut p = 0;
    for l in 0..i {
        for m in (l + 1)..i {
            if word[l] != "null" {
                if word[l] == word[m] {
                    word[m] = "null".to_string();
                } else {
                    break;
                }
            }
        }
    }

    for l in 0..i {
        if word[l] != "null" {
            dict.push(word[l].clone());
            p += 1;
        } else {
            mark[l] = 100;
        }
    }

    let mut r = 0; // Current word's duplicates
    let mut s = 0; // Total duplicates
    for k in 0..i {
        if mark[k] == 0 {
            r = 0;
            page[k - s][0] = num[k];
        } else {
            r += 1;
            s += 1;
            page[k - s][r] = num[k];
        }
    }

    for k in 0..(i - s) {
        for t in 0..i {
            if page[k][t] > page[k][t + 1] {
                page[k].swap(t, t + 1);
                t = 0; // Restart sorting
            } else if page[k][t] == 0 {
                break;
            }
        }
    }

    for k in 0..(i - s) {
        println!("{}", dict[k]);
        for t in 1..i {
            if t == 1 {
                print!("{}", page[k][t]);
            } else if page[k][t] != 0 {
                print!(" {}", page[k][t]);
            } else {
                println!();
                break;
            }
        }
    }
}
