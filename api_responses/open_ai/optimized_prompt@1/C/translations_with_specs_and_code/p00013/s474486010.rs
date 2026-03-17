use std::io::{self, BufRead};

fn main() {
    let mut railroad: Vec<i32> = Vec::new(); // Vector to store car identifiers
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let car: i32 = line.unwrap().trim().parse().unwrap(); // Read and parse input

        if car == 0 { // Car exiting
            if let Some(removed_car) = railroad.pop() { // Remove the last car if available
                println!("{}", removed_car); // Output the identifier of the car that exited
            }
        } else { // Car entering
            if railroad.len() < 10 { // Check if we can add more cars
                railroad.push(car); // Add the car to storage
            }
            // If we reach the limit of 10 cars, we do nothing (undefined behavior as per spec)
        }
    }
}
```

### Notes:
- The Rust code uses a `Vec<i32>` to dynamically manage the storage of car identifiers, which allows for easy addition and removal of cars.
- The input is read line by line, and each line is parsed into an integer. The program assumes valid input as per the specification.
- When a car exits (indicated by a zero), the program checks if there is a car to remove using `pop()`, which safely handles the case where there are no cars in storage.
- The maximum capacity of ten cars is enforced by checking the length of the vector before adding a new car.
- The behavior when trying to add more than ten cars is left undefined, as specified.
