use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut data = [[0i64; 4]; 4000];
    let mut kaunto = [[0; 2]; 4000];
    let mut kazu: i32;
    
    // Initialize data and kaunto arrays
    for i in 0..4000 {
        data[i][0] = (i + 1) as i64;
        kaunto[i][0] = i + 1;
        for j in 1..4 {
            data[i][j] = 0;
        }
        kaunto[i][1] = 0;
    }

    let mut input = stdin.lock().lines();
    
    // Read the first value of kazu
    kazu = input.next().unwrap().unwrap().trim().parse().unwrap();
    
    while kazu != 0 {
        let mut count = 0;

        for _ in 0..kazu {
            let number: usize = input.next().unwrap().unwrap().trim().parse().unwrap();
            let values: Vec<i64> = input.next().unwrap().unwrap().trim().split_whitespace()
                .map(|s| s.parse().unwrap()).collect();
            data[number - 1][1] = values[0];
            data[number - 1][2] = values[1];
            data[number - 1][3] += data[number - 1][1] * data[number - 1][2];
        }

        for i in 0..4000 {
            if data[i][3] >= 1_000_000 && kaunto[i][1] == 0 {
                println!("{}", kaunto[i][0]);
                kaunto[i][1] = 1;
                count = 1;
            }
        }
        
        if count == 0 {
            println!("NA");
        }

        // Read the next value of kazu
        kazu = input.next().unwrap().unwrap().trim().parse().unwrap();
    }
}
