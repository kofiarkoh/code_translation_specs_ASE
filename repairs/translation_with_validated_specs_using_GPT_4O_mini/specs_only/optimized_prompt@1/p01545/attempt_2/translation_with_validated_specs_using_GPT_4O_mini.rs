struct SegmentTree {
    data: Vec<i32>,
    size: usize,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        let data = vec![i32::MIN; 2 * size];
        SegmentTree { data, size }
    }

    fn update(&mut self, index: usize, value: i32) {
        let mut idx = index + self.size;
        self.data[idx] = value;
        while idx > 1 {
            idx /= 2;
            self.data[idx] = self.data[2 * idx].max(self.data[2 * idx + 1]);
        }
    }

    fn query(&self, left: usize, right: usize) -> i32 {
        let mut result = i32::MIN;
        let mut l = left + self.size;
        let mut r = right + self.size;

        while l < r {
            if l % 2 == 1 {
                result = result.max(self.data[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                result = result.max(self.data[r]);
            }
            l /= 2;
            r /= 2;
        }
        result
    }
}

struct HouseMover {
    positions: Vec<i32>,
    segment_tree: SegmentTree,
}

impl HouseMover {
    fn new(positions: Vec<i32>) -> Self {
        let n = positions.len();
        let mut segment_tree = SegmentTree::new(n);
        for (i, &pos) in positions.iter().enumerate() {
            segment_tree.update(i, pos);
        }
        HouseMover { positions, segment_tree }
    }

    fn total_cost(&mut self) -> i64 {
        let n = self.positions.len() as i64;
        let expected_cost = n * (n + 1) / 2; // Sum of first n natural numbers
        let mut cumulative_max = 0;

        for i in 0..n {
            let max_value = self.segment_tree.query(0, i as usize + 1);
            cumulative_max += max_value as i64;
            // Update the segment tree with the current position
            self.segment_tree.update(i as usize, self.positions[i as usize]);
        }

        // The total cost is the expected cost minus the cumulative maximum
        expected_cost - cumulative_max
    }
}

fn main() {
    let n = 5; // Example number of houses
    let positions = vec![1, 2, 3, 4, 5]; // Example positions
    let mut mover = HouseMover::new(positions);
    let cost = mover.total_cost();
    println!("{}", cost); // Output the total cost
}
