use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        
        let t0 = iter.next().unwrap();
        let t1 = iter.next().unwrap();
        let t2 = iter.next().unwrap();

        // Termination condition
        if t0 == 0 || t1 == 0 || t2 == 0 {
            break;
        }

        let mut n: i32 = iter.next().unwrap();

        let mut adjusted_t0 = t0;
        let mut adjusted_t1 = t1;
        let mut adjusted_t2 = t2;

        // Adjusting t0 and t1 based on t2
        if adjusted_t0 >= adjusted_t2 {
            adjusted_t0 = adjusted_t2;
            adjusted_t2 = -1;
        }
        if adjusted_t1 >= adjusted_t2 && adjusted_t2 != -1 {
            adjusted_t1 = adjusted_t2;
        }

        let hakari = adjusted_t0 * adjusted_t0 + adjusted_t1 * adjusted_t1;

        for _ in 0..n {
            let mut data_input = String::new();
            reader.read_line(&mut data_input).unwrap();
            let data: i32 = data_input.trim().parse().unwrap();
            let data_squared = (data * 2).pow(2); // Square of double the input

            // Output based on comparison
            if hakari < data_squared {
                println!("OK");
            } else {
                println!("NA");
            }
        }
    }
}
