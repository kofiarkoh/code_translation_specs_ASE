use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Segment {
    start: Point,
    end: Point,
}

impl Segment {
    // Check if two segments intersect
    fn intersects(&self, other: &Segment) -> bool {
        // Using orientation method to check for intersection
        fn orientation(p: &Point, q: &Point, r: &Point) -> i32 {
            let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
            if val == 0 { 0 } // collinear
            else if val > 0 { 1 } // clockwise
            else { 2 } // counterclockwise
        }

        let o1 = orientation(&self.start, &self.end, &other.start);
        let o2 = orientation(&self.start, &self.end, &other.end);
        let o3 = orientation(&other.start, &other.end, &self.start);
        let o4 = orientation(&other.start, &other.end, &self.end);

        // General case
        if o1 != o2 && o3 != o4 {
            return true;
        }

        false // Doesn't intersect
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of segments
    let lim: usize = lines.next().unwrap().unwrap().parse().unwrap();
    if lim == 0 {
        println!("0");
        return;
    }

    let mut segments = Vec::new();

    // Read segments
    for _ in 0..lim {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<f64> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        segments.push(Segment {
            start: Point { x: coords[0], y: coords[1] },
            end: Point { x: coords[2], y: coords[3] },
        });
    }

    // Build the graph based on intersections
    let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();

    for i in 0..segments.len() {
        for j in (i + 1)..segments.len() {
            if segments[i].intersects(&segments[j]) {
                graph.entry(i).or_insert_with(HashSet::new).insert(j);
                graph.entry(j).or_insert_with(HashSet::new).insert(i);
            }
        }
    }

    // Welsh-Powell algorithm for graph coloring
    let mut colors = vec![0; segments.len()];
    let mut color_count = 0;

    let mut sorted_vertices: Vec<usize> = (0..segments.len()).collect();
    sorted_vertices.sort_by_key(|&i| graph.get(&i).map_or(0, |adj| adj.len()));

    for vertex in sorted_vertices.iter().rev() {
        if colors[*vertex] == 0 {
            color_count += 1;
            let mut forbidden_colors = HashSet::new();

            // Mark colors of adjacent vertices
            if let Some(adj) = graph.get(vertex) {
                for &adj_vertex in adj {
                    if colors[adj_vertex] != 0 {
                        forbidden_colors.insert(colors[adj_vertex]);
                    }
                }
            }

            // Assign the lowest available color
            let mut color = 1;
            while forbidden_colors.contains(&color) {
                color += 1;
            }
            colors[*vertex] = color;
        }
    }

    println!("{}", color_count);
}
