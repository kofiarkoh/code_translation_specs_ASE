use std::collections::HashMap;

const MAX_ENERGY: usize = 100;

fn min_items_to_target(target: usize, items: &[(usize, usize, usize)]) -> Result<usize, &'static str> {
    if target == 0 {
        return Ok(0); // No items needed to reach a target of 0
    }
    if items.is_empty() {
        return Err("NA"); // No items available
    }

    let mut memo: HashMap<(usize, usize), Option<usize>> = HashMap::new();
    
    fn search(
        target: usize,
        energy: usize,
        items: &[(usize, usize, usize)],
        memo: &mut HashMap<(usize, usize), Option<usize>>,
    ) -> Option<usize> {
        if target <= 0 {
            return Some(0); // Target reached
        }
        if energy > MAX_ENERGY {
            return None; // Exceeded energy limit
        }
        if let Some(&result) = memo.get(&(target, energy)) {
            return result; // Return cached result
        }

        let mut min_count: Option<usize> = None; // Explicit type annotation

        for &(value, cost, required) in items {
            if energy >= required {
                if let Some(count) = search(target.saturating_sub(value), energy.saturating_sub(cost), items, memo) {
                    let total_count = count + 1;
                    min_count = match min_count {
                        Some(current_min) => Some(current_min.min(total_count)),
                        None => Some(total_count),
                    };
                }
            }
        }

        memo.insert((target, energy), min_count);
        min_count
    }

    let result = search(target, MAX_ENERGY, items, &mut memo);
    match result {
        Some(count) => Ok(count),
        None => Err("NA"),
    }
}

fn main() {
    let target = 10; // Example target value
    let items = vec![(5, 2, 1), (3, 1, 1)]; // Example items (value, energy cost, required energy)

    match min_items_to_target(target, &items) {
        Ok(count) => println!("{}", count),
        Err(msg) => println!("{}", msg),
    }
}
