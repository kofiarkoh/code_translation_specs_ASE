use std::collections::HashMap;
use std::io;

fn floyd_warshall(n: usize, edges: &[(usize, usize, usize)]) -> Vec<Vec<usize>> {
    let mut dist = vec![vec![usize::MAX; n]; n];

    // Initialize distances based on edges
    for (a, b, c) in edges {
        dist[*a - 1][*b - 1] = (*c).min(dist[*a - 1][*b - 1]);
        dist[*b - 1][*a - 1] = (*c).min(dist[*b - 1][*a - 1]); // Undirected graph
    }

    // Distance to self is zero
    for i in 0..n {
        dist[i][i] = 0;
    }

    // Floyd-Warshall algorithm
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] != usize::MAX && dist[k][j] != usize::MAX {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }
    }

    dist
}

fn dfs(locations: &[usize], dist: &Vec<Vec<usize>>, visited: &mut Vec<bool>, current: usize, count: usize, total: usize, min_distance: &mut usize) {
    if count == locations.len() {
        // Return to starting point
        let return_distance = dist[current][locations[0] - 1];
        *min_distance = (*min_distance).min(total + return_distance);
        return;
    }

    for i in 0..locations.len() {
        if !visited[i] {
            visited[i] = true;
            let next = locations[i] - 1; // Convert to 0-based index
            dfs(locations, dist, visited, next, count + 1, total + dist[current][next], min_distance);
            visited[i] = false; // Backtrack
        }
    }
}

fn minimum_distance(n: usize, edges: &[(usize, usize, usize)], locations: &[usize]) -> usize {
    let dist = floyd_warshall(n, edges);
    let mut min_distance = usize::MAX;
    let mut visited = vec![false; locations.len()];

    // Start DFS from the first location
    visited[0] = true;
    dfs(locations, &dist, &mut visited, locations[0] - 1, 1, 0, &mut min_distance);

    min_distance
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let m: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let r: usize = input.trim().parse().unwrap();

    let mut locations = Vec::new();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    for loc in input.trim().split_whitespace() {
        locations.push(loc.parse::<usize>().unwrap());
    }

    let mut edges = Vec::new();
    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let edge: Vec<usize> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        edges.push((edge[0], edge[1], edge[2]));
    }

    let result = minimum_distance(n, &edges, &locations);
    println!("{}", result);
}
