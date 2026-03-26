use std::io::{self, BufRead};

#[derive(Debug)]
struct Artwork {
    value: i64, // long long
    weight: i32, // int
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of artworks
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for N");
    
    // Ensure N is greater than 0
    if n == 0 {
        panic!("N must be greater than 0");
    }

    let mut artworks: Vec<Artwork> = Vec::with_capacity(n);

    // Read artworks
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<i64> = line.split_whitespace()
            .map(|s| s.parse().expect("Invalid input for artwork value or weight"))
            .collect();
        artworks.push(Artwork {
            value: parts[0], // tbl[i].a
            weight: parts[1] as i32, // tbl[i].b
        });
    }

    // Sort artworks by value
    artworks.sort_by_key(|art| art.value);

    let mut max_difference = i64::MIN; // Initialize to the smallest possible value
    let mut cumulative_weight = 0;
    let mut cumulative_value = 0;

    // Calculate the maximum difference
    for artwork in artworks {
        cumulative_weight += artwork.weight;
        cumulative_value += artwork.value;
        let current_difference = cumulative_weight - cumulative_value;
        if current_difference > max_difference {
            max_difference = current_difference;
        }
    }

    // Print the result
    println!("{}", max_difference);
}
