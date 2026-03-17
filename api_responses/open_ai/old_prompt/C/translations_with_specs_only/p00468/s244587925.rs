fn main() {
    // Define the maximum number of students
    const MAX_STUDENTS: usize = 501;

    // Adjacency matrix to represent the graph
    let mut student: [[bool; MAX_STUDENTS]; MAX_STUDENTS] = [[false; MAX_STUDENTS]; MAX_STUDENTS];
    // List to track reachability from student 1
    let mut list: [bool; MAX_STUDENTS] = [false; MAX_STUDENTS];

    // Read input until termination condition (n = 0 and m = 0)
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        
        // Read n and m
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();

        // Termination condition
        if n == 0 && m == 0 {
            break;
        }

        // Reset the adjacency matrix and reachability list for each new input
        for i in 1..=n {
            for j in 1..=n {
                student[i][j] = false;
            }
            list[i] = false;
        }

        // Read m pairs of connections
        for _ in 0..m {
            let p1: usize = iter.next().unwrap().parse().unwrap();
            let p2: usize = iter.next().unwrap().parse().unwrap();
            // Update the adjacency matrix
            student[p1][p2] = true;
            student[p2][p1] = true; // Undirected graph
        }

        // Perform a DFS or BFS to count reachable students from student 1
        let mut count = 0;
        let mut stack = vec![1]; // Start from student 1
        list[1] = true; // Mark student 1 as visited

        while let Some(current) = stack.pop() {
            count += 1; // Count the current student
            for neighbor in 1..=n {
                if student[current][neighbor] && !list[neighbor] {
                    list[neighbor] = true; // Mark as visited
                    stack.push(neighbor); // Add to stack for further exploration
                }
            }
        }

        // Output the count of students connected to student 1
        println!("{}", count);
    }
}
