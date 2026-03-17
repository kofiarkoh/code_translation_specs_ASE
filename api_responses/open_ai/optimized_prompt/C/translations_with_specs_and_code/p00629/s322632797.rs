use std::io::{self, BufRead};

#[derive(Debug)]
struct Record {
    i: i32,
    u: i32,
    a: i32,
    p: i32,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut records: Vec<Record> = Vec::new();
    let mut frequency: [i32; 1001] = [0; 1001];
    let mut output: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let n: usize = line.trim().parse().unwrap();

        if n == 0 {
            break;
        }

        records.clear();
        frequency.fill(0);

        for _ in 0..n {
            let line = reader.lines().next().unwrap().unwrap();
            let values: Vec<i32> = line
                .trim()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            records.push(Record {
                i: values[0],
                u: values[1],
                a: values[2],
                p: values[3],
            });
        }

        // Sort records based on the specified criteria
        records.sort_by(|r1, r2| {
            let a_cmp = r2.a.cmp(&r1.a);
            if a_cmp != std::cmp::Ordering::Equal {
                return a_cmp;
            }
            let p_cmp = r1.p.cmp(&r2.p);
            if p_cmp != std::cmp::Ordering::Equal {
                return p_cmp;
            }
            r1.i.cmp(&r2.i)
        });

        let mut head = 0;

        for record in records.iter() {
            frequency[record.u as usize] += 1;

            if head < 10 && frequency[record.u as usize] <= 3 {
                output.push(record.i);
                head += 1;
            } else if head < 20 && frequency[record.u as usize] <= 2 {
                output.push(record.i);
                head += 1;
            } else if head < 26 && frequency[record.u as usize] <= 1 {
                output.push(record.i);
                head += 1;
            }
        }

        for id in output.iter() {
            println!("{}", id);
        }

        output.clear(); // Clear output for the next dataset
    }
}
