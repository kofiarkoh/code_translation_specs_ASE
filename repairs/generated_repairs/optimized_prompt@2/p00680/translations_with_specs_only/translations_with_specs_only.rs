#[derive(Debug, Clone)]
struct Edge {
    destination: usize,
    capacity: i32,
}

#[derive(Debug)]
struct Graph {
    edges: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(num_nodes: usize) -> Self {
        Graph {
            edges: vec![Vec::new(); num_nodes],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, capacity: i32) {
        self.edges[from].push(Edge { destination: to, capacity });
    }
}

fn gauss_jordan(matrix: &mut Vec<Vec<f64>>, constants: &mut Vec<f64>) -> Result<(), &'static str> {
    let n = matrix.len();
    for i in 0..n {
        // Find pivot
        let pivot_row = (i..n).find(|&r| matrix[r][i] != 0.0);
        match pivot_row {
            Some(p) => {
                matrix.swap(i, p);
                let pivot = matrix[i][i];
                for j in 0..n {
                    matrix[i][j] /= pivot;
                }
                constants[i] /= pivot;

                for k in 0..n {
                    if k != i {
                        let factor = matrix[k][i];
                        for j in 0..n {
                            matrix[k][j] -= factor * matrix[i][j];
                        }
                        constants[k] -= factor * constants[i];
                    }
                }
            }
            None => return Err("Gauss-Jordan elimination failed: pivot is zero"),
        }
    }
    Ok(())
}

fn min_cost_flow(graph: &Graph, source: usize, sink: usize, flow: i32) -> Result<f64, &'static str> {
    // Placeholder for min-cost flow algorithm
    // This should implement a modified Dijkstra's approach
    // For now, we return -1 to indicate "impossible"
    Err("impossible")
}

fn process_test_case(
    num_nodes: usize,
    source: usize,
    sink: usize,
    flow: i32,
    matrix: Vec<Vec<f64>>,
    constants: Vec<f64>,
    edges: Vec<(usize, usize, i32)>,
) -> String {
    if flow < 0 {
        return "impossible".to_string();
    }

    let mut graph = Graph::new(num_nodes);
    for (from, to, capacity) in edges {
        graph.add_edge(from, to, capacity);
    }

    let mut matrix_clone = matrix.clone();
    let mut constants_clone = constants.clone();

    if gauss_jordan(&mut matrix_clone, &mut constants_clone).is_err() {
        return "impossible".to_string();
    }

    match min_cost_flow(&graph, source, sink, flow) {
        Ok(cost) => format!("{:.10}", cost),
        Err(_) => "impossible".to_string(),
    }
}

fn main() {
    // Example usage
    let num_test_cases = 1; // This would be read from input
    let test_cases = vec![
        (
            3, // num_nodes
            0, // source
            2, // sink
            5, // flow
            vec![vec![1.0, 2.0, 3.0], vec![0.0, 1.0, 4.0], vec![0.0, 0.0, 1.0]], // matrix
            vec![1.0, 2.0, 3.0], // constants
            vec![(0, 1, 10), (1, 2, 10)], // edges
        ),
    ];

    for (num_nodes, source, sink, flow, matrix, constants, edges) in test_cases {
        let result = process_test_case(num_nodes, source, sink, flow, matrix, constants, edges);
        println!("{}", result);
    }
}
