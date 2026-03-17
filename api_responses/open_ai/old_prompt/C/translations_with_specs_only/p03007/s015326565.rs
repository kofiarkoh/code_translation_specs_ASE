fn process_array(n: usize, a: Vec<i64>) {
    // Step 1: Sort the array
    let mut sorted_a = a.clone();
    sorted_a.sort();

    // Step 2: Compute the sum of absolute values
    let sum: i64 = sorted_a.iter().map(|&x| x.abs()).sum();

    // Output the computed sum
    println!("{}", sum);

    // Step 3: Output pairs of values based on their signs
    let mut positive_pairs = Vec::new();
    let mut negative_pairs = Vec::new();

    for &value in &sorted_a {
        if value > 0 {
            positive_pairs.push(value);
        } else if value < 0 {
            negative_pairs.push(value);
        }
    }

    // Output pairs of positive values
    for i in 0..positive_pairs.len() {
        for j in (i + 1)..positive_pairs.len() {
            println!("{} {}", positive_pairs[i], positive_pairs[j]);
        }
    }

    // Output pairs of negative values
    for i in 0..negative_pairs.len() {
        for j in (i + 1)..negative_pairs.len() {
            println!("{} {}", negative_pairs[i], negative_pairs[j]);
        }
    }

    // Output pairs of one positive and one negative value
    for &pos in &positive_pairs {
        for &neg in &negative_pairs {
            println!("{} {}", pos, neg);
        }
    }
}

fn main() {
    // Example usage
    let n = 5; // Number of integers
    let a = vec![-1, 2, -3, 4, 5]; // Example array
    process_array(n, a);
}
