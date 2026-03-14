use std::io::{self, BufRead};

fn main() {
    const MAX_ENTRIES: usize = 4000;
    let mut data = [[0; 4]; MAX_ENTRIES];
    let mut kaunto = [[0; 2]; MAX_ENTRIES];

    // Initialize data and kaunto arrays
    for i in 0..MAX_ENTRIES {
        data[i][0] = (i + 1) as i64; // Using i64 to match long long
        kaunto[i][0] = (i + 1) as i64;
        data[i][1] = 0;
        data[i][2] = 0;
        kaunto[i][1] = 0;
    }

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let kazu: usize = iter.next().unwrap().parse().unwrap();

        if kazu == 0 {
            break;
        }

        let mut count = 0;

        for _ in 0..kazu {
            let number: usize = iter.next().unwrap().parse().unwrap();
            let value1: i64 = iter.next().unwrap().parse().unwrap();
            let value2: i64 = iter.next().unwrap().parse().unwrap();

            // Update the data structure
            data[number - 1][1] = value1;
            data[number - 1][2] = value2;
            data[number - 1][3] += value1 * value2;
        }

        for i in 0..MAX_ENTRIES {
            if data[i][3] >= 1_000_000 && kaunto[i][1] == 0 {
                println!("{}", kaunto[i][0]);
                kaunto[i][1] = 1;
                count = 1;
            }
        }

        if count == 0 {
            println!("NA");
        }
    }
}
