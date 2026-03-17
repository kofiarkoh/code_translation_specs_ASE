fn main() {
    // Read input from standard input
    let input = std::io::stdin().read_line().expect("Failed to read line");
    let mut parts = input.split_whitespace();

    // Parse the two integers n (balls) and k (boxes)
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    // Initialize the dynamic programming table
    let mut pdp = vec![vec![0; k + 1]; n + 1];

    // Base case: There is one way to partition 0 balls into 0 boxes
    pdp[0][0] = 1;

    // Fill the dynamic programming table
    for i in 0..=n {
        for j in 1..=k {
            // The number of ways to partition i balls into j boxes
            pdp[i][j] = pdp[i][j - 1]; // Ways without using the j-th box
            if i > 0 {
                pdp[i][j] += pdp[i - 1][j]; // Ways using the j-th box
            }
        }
    }

    // Output the result: number of ways to partition n balls into k boxes
    println!("{}", pdp[n][k]);
}
