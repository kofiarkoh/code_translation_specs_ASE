use std::io::{self, BufRead};

struct RailroadTrack {
    cars: Vec<i32>, // Internal record of cars on the track
}

impl RailroadTrack {
    fn new() -> Self {
        RailroadTrack { cars: Vec::new() }
    }

    fn enter_car(&mut self, car_number: i32) {
        // Add the car to the track if it's a positive integer
        if car_number > 0 && self.cars.len() < 10 {
            self.cars.push(car_number);
        }
    }

    fn exit_car(&mut self) -> Option<i32> {
        // Remove the most recently added car if there is one
        if let Some(car) = self.cars.pop() {
            return Some(car);
        }
        // If no cars are present, return None (undefined behavior as per spec)
        None
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut track = RailroadTrack::new();

    for line in handle.lines() {
        if let Ok(input) = line {
            if let Ok(command) = input.trim().parse::<i32>() {
                if command == 0 {
                    // Exit command
                    if let Some(car) = track.exit_car() {
                        println!("{}", car); // Output the car number that exits
                    }
                    // If no cars are present, do nothing (undefined behavior)
                } else {
                    // Enter command (positive integer)
                    track.enter_car(command);
                }
            }
            // The program does not handle invalid input or provide feedback for erroneous commands
        }
    }
}
