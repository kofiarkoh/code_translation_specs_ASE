// Book replacement system for students using a fixed number of desks
fn main() {
    // Example input: number of desks, capacity of each desk, number of students
    let test_cases = vec![
        (3, 2, 5, vec![1, 2, 3, 1, 2]), // Example test case
    ];

    for (desks, capacity, students, requests) in test_cases {
        // Validate inputs
        if desks <= 0 || capacity <= 0 || students <= 0 {
            // Terminate if any input is invalid
            return;
        }

        // Initialize desks
        let mut desk_books = vec![vec![]; desks];
        let mut total_cost = 0;

        // Process each student's requests
        for request in requests {
            let desk_index = (request % desks as i32) as usize; // Simple desk assignment based on book ID
            if desk_books[desk_index].len() < capacity as usize {
                // If there's space on the desk, add the book
                desk_books[desk_index].push(request);
            } else {
                // If the desk is full, replace the first book (FIFO)
                desk_books[desk_index].remove(0); // Remove the oldest book
                desk_books[desk_index].push(request); // Add the new book
                total_cost += 1; // Increment cost for replacement
            }
        }

        // Output the total cost for the current test case
        println!("{}", total_cost);
    }
}
