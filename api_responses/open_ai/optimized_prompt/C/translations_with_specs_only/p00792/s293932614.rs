// Define the maximum bounds for the simulation space
const MAX_COORDINATE: i32 = 1000; // Example maximum coordinate limit

// Define the structure for a mirror
struct Mirror {
    orientation: char, // 'x' for vertical, any other character for horizontal
    x: i32,
    y: i32,
}

// Define the main simulation structure
struct Simulation {
    mirrors: Vec<Mirror>,
    position: (i32, i32), // Entity's position in 2D space
    direction: (i32, i32), // Direction of movement, (dx, dy)
}

impl Simulation {
    // Create a new simulation with an initial position and direction
    fn new() -> Self {
        Self {
            mirrors: Vec::new(),
            position: (0, 0), // Starting at origin
            direction: (1, 0), // Initial direction to the right
        }
    }

    // Add a mirror to the simulation if within bounds
    fn add_mirror(&mut self, orientation: char, x: i32, y: i32) {
        if x >= 0 && x <= MAX_COORDINATE && y >= 0 && y <= MAX_COORDINATE {
            self.mirrors.push(Mirror { orientation, x, y });
        }
        // If out of bounds, do nothing as per specification
    }

    // Simulate the movement of the entity
    fn move_entity(&mut self) {
        // Update position based on current direction
        self.position.0 += self.direction.0;
        self.position.1 += self.direction.1;

        // Check for wall collisions and reflect if necessary
        if self.position.0 < 0 || self.position.0 > MAX_COORDINATE || 
           self.position.1 < 0 || self.position.1 > MAX_COORDINATE {
            // Reflect direction upon hitting a wall
            self.direction.0 = -self.direction.0;
            self.direction.1 = -self.direction.1;
            // Keep the entity within bounds
            self.position.0 = self.position.0.clamp(0, MAX_COORDINATE);
            self.position.1 = self.position.1.clamp(0, MAX_COORDINATE);
        }

        // Check for mirror interactions
        for mirror in &self.mirrors {
            if (self.position.0 == mirror.x && self.position.1 == mirror.y) {
                // Reflect based on mirror orientation
                if mirror.orientation == 'x' {
                    self.direction.0 = -self.direction.0; // Reflect horizontally
                } else {
                    self.direction.1 = -self.direction.1; // Reflect vertically
                }
                break; // Only interact with one mirror at a time
            }
        }
    }

    // Get the final coordinates of the entity
    fn get_final_coordinates(&self) -> (i32, i32) {
        self.position
    }
}

// Main function to run the simulation
fn main() {
    let mut simulation = Simulation::new();
    
    // Example input processing (this would be replaced with actual input handling)
    let number_of_mirrors = 3; // Example input
    let mirrors_data = vec![
        ('x', 5, 5),
        ('y', 10, 10),
        ('x', 15, 15),
    ];

    // Process mirrors
    for (orientation, x, y) in mirrors_data {
        simulation.add_mirror(orientation, x, y);
    }

    // Simulate movements (this would be based on actual movement logic)
    for _ in 0..10 { // Example movement iterations
        simulation.move_entity();
    }

    // Output final coordinates
    let (final_x, final_y) = simulation.get_final_coordinates();
    println!("Final coordinates: ({}, {})", final_x, final_y);
}
