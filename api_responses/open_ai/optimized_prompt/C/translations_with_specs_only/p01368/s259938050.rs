// Import necessary libraries
use std::collections::HashMap;
use std::cmp::Ordering;

// Define a structure to represent a connection between locations
#[derive(Debug)]
struct Connection {
    from: usize,
    to: usize,
    distance: usize,
}

// Define a structure to represent a task
#[derive(Debug)]
struct Task {
    location: usize,
    start_time: usize,
}

// Function to compute the maximum number of non-overlapping tasks
fn max_non_overlapping_tasks(locations: usize, connections: usize, tasks: usize, conn_list: Vec<Connection>, task_list: Vec<Task>) -> usize {
    // Initialize the distance matrix with infinity
    let mut distance = vec![vec![usize::MAX; locations]; locations];
    
    // Set the distance from each location to itself to 0
    for i in 0..locations {
        distance[i][i] = 0;
    }

    // Populate the distance matrix based on connections
    for conn in conn_list {
        distance[conn.from][conn.to] = conn.distance;
        distance[conn.to][conn.from] = conn.distance; // Assuming undirected connections
    }

    // Apply the Floyd-Warshall algorithm to find shortest paths
    for k in 0..locations {
        for i in 0..locations {
            for j in 0..locations {
                if distance[i][k] != usize::MAX && distance[k][j] != usize::MAX {
                    distance[i][j] = distance[i][j].min(distance[i][k] + distance[k][j]);
                }
            }
        }
    }

    // Sort tasks based on their start time
    let mut sorted_tasks = task_list.clone();
    sorted_tasks.sort_by(|a, b| a.start_time.cmp(&b.start_time));

    // Schedule tasks while respecting time constraints
    let mut count = 0;
    let mut last_end_time = 0;

    for task in sorted_tasks {
        // Check if the task can be scheduled
        if task.start_time >= last_end_time {
            count += 1;
            last_end_time = task.start_time + distance[task.location][task.location]; // Assuming task duration is equal to distance to itself
        }
    }

    count
}

// Main function to handle input and output
fn main() {
    // Example input (this would typically come from user input or a file)
    let locations = 3;
    let connections = 3;
    let tasks = 3;

    let conn_list = vec![
        Connection { from: 0, to: 1, distance: 2 },
        Connection { from: 1, to: 2, distance: 3 },
        Connection { from: 0, to: 2, distance: 5 },
    ];

    let task_list = vec![
        Task { location: 0, start_time: 0 },
        Task { location: 1, start_time: 2 },
        Task { location: 2, start_time: 5 },
    ];

    // Compute the maximum number of non-overlapping tasks
    let result = max_non_overlapping_tasks(locations, connections, tasks, conn_list, task_list);
    
    // Output the result
    println!("{}", result);
}
