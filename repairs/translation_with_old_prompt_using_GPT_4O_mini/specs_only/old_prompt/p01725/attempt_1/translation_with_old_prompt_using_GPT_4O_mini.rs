use std::cmp;

fn evaluate_expression(expr: &str) -> i64 {
    // This function evaluates the expression and returns the maximum value
    // by considering all possible operator precedence arrangements.
    
    let mut max_value = i64::MIN;
    
    // Iterate through the expression to find operators
    for i in 0..expr.len() {
        let c = expr.chars().nth(i).unwrap();
        
        // If the character is an operator, we split the expression
        if c == '+' || c == '-' || c == '*' {
            // Split the expression into left and right parts
            let left = &expr[0..i];
            let right = &expr[i + 1..];
            
            // Evaluate both sides recursively
            let left_value = evaluate_expression(left);
            let right_value = evaluate_expression(right);
            
            // Calculate the result based on the operator
            let result = match c {
                '+' => left_value + right_value,
                '-' => left_value - right_value,
                '*' => left_value * right_value,
                _ => unreachable!(),
            };
            
            // Update the maximum value found
            max_value = cmp::max(max_value, result);
        }
    }
    
    // If no operator was found, it means we have a number
    if max_value == i64::MIN {
        // Parse the number from the expression
        max_value = expr.parse::<i64>().unwrap();
    }
    
    max_value
}

fn main() {
    let input = "3-2*2"; // Example input, replace with actual input reading
    let result = evaluate_expression(input);
    println!("{}", result); // Output the maximum value
}
