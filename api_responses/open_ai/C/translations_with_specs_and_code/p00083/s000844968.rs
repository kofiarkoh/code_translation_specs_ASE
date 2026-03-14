use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() != 3 {
            continue; // Skip invalid input lines
        }

        let year: i32 = parts[0].parse().unwrap();
        let month: i32 = parts[1].parse().unwrap();
        let day: i32 = parts[2].parse().unwrap();

        let mut era_printed = false;

        if year < 1868 || (year == 1868 && (month < 9 || (month == 9 && day < 8))) {
            println!("pre-meiji");
            era_printed = true;
        } else if year < 1912 || (year == 1912 && (month < 7 || (month == 7 && day < 30))) {
            println!("meiji {}", year - 1868 + 1);
            era_printed = true;
        } else if year < 1926 || (year == 1926 && (month < 12 || (month == 12 && day < 25))) {
            println!("taisyo {}", year - 1912 + 1);
            era_printed = true;
        } else if year < 1989 || (year == 1989 && (month < 1 || (month == 1 && day < 8))) {
            println!("showa {}", year - 1926 + 1);
            era_printed = true;
        } else {
            println!("heisei {}", year - 1989 + 1);
            era_printed = true;
        }

        if !era_printed {
            println!("{} {} {}", year, month, day);
        }
    }
}
