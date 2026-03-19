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
    let mut selected_ids: Vec<i32> = Vec::new();

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
            if r1.a != r2.a {
                r2.a.cmp(&r1.a) // Sort by A descending
            } else if r1.p != r2.p {
                r1.p.cmp(&r2.p) // Sort by P ascending
            } else {
                r1.i.cmp(&r2.i) // Sort by I ascending
            }
        });

        let mut head = 0;

        for record in records.iter() {
            frequency[record.u as usize] += 1;

            if head < 10 && frequency[record.u as usize] <= 3 {
                selected_ids.push(record.i);
                head += 1;
            } else if head < 20 && frequency[record.u as usize] <= 2 {
                selected_ids.push(record.i);
                head += 1;
            } else if head < 26 && frequency[record.u as usize] <= 1 {
                selected_ids.push(record.i);
                head += 1;
            }
        }

        for id in selected_ids.iter() {
            println!("{}", id);
        }

        selected_ids.clear(); // Clear for the next dataset
    }
}
