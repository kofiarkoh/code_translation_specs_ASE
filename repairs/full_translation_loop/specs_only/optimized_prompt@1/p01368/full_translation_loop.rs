#[derive(Debug, Clone)]
struct Connection {
    from: usize,
    to: usize,
    distance: usize,
}

#[derive(Debug, Clone)]
struct Task {
    start_location: usize,
    start_time: usize,
}

fn floyd_warshall(distances: &mut Vec<Vec<usize>>, num_locations: usize) {
    for k in 0..num_locations {
        for i in 0..num_locations {
            for j in 0..num_locations {
                if distances[i][j] > distances[i][k].saturating_add(distances[k][j]) {
                    distances[i][j] = distances[i][k].saturating_add(distances[k][j]);
                }
            }
        }
    }
}

fn max_non_overlapping_tasks(
    num_locations: usize,
    connections: Vec<Connection>,
    tasks: Vec<Task>,
) -> usize {
    // Initialize distance matrix
    let mut distances = vec![vec![usize::MAX; num_locations]; num_locations];
    for i in 0..num_locations {
        distances[i][i] = 0; // Distance to self is zero
    }

    // Fill in the distance matrix with the given connections
    for conn in connections {
        distances[conn.from][conn.to] = conn.distance;
        distances[conn.to][conn.from] = conn.distance; // Assuming undirected connections
    }

    // Compute shortest paths
    floyd_warshall(&mut distances, num_locations);

    // Sort tasks by start time
    let mut sorted_tasks = tasks.clone();
    sorted_tasks.sort_by_key(|task| task.start_time);

    let mut count = 0;
    let mut last_end_time = 0;

    for task in sorted_tasks {
        // Check if the task can be scheduled
        if task.start_time >= last_end_time {
            count += 1;
            last_end_time = task.start_time + 1; // Assuming each task takes 1 time unit
        }
    }

    count
}

fn main() {
    // Example usage
    let num_locations = 4;
    let connections = vec![
        Connection { from: 0, to: 1, distance: 2 },
        Connection { from: 0, to: 2, distance: 4 },
        Connection { from: 1, to: 2, distance: 1 },
        Connection { from: 2, to: 3, distance: 1 },
    ];
    let tasks = vec![
        Task { start_location: 0, start_time: 5 },
        Task { start_location: 1, start_time: 7 },
        Task { start_location: 2, start_time: 8 },
    ];

    let result = max_non_overlapping_tasks(num_locations, connections, tasks);
    println!("{}", result); // Output the maximum number of non-overlapping tasks
}
