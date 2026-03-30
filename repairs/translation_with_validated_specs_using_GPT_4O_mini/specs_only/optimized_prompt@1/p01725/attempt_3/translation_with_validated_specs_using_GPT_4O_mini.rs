use std::collections::HashMap;

pub fn max_expression_value(expression: &str) -> i64 {
    // Memoization to store results of sub-expressions
    let mut memo: HashMap<String, i64> = HashMap::new();
    evaluate(&expression, &mut memo)
}

fn evaluate(expr: &str, memo: &mut HashMap<String, i64>) -> i64 {
    // Check if the result for this expression is already computed
    if let Some(&result) = memo.get(expr) {
        return result;
    }

    let mut max_value = i64::MIN;
    let mut current_number = 0;
    let mut operators: Vec<char> = Vec::new();
    let mut numbers: Vec<i64> = Vec::new();

    let mut i = 0;
    while i < expr.len() {
        let c = expr.chars().nth(i).unwrap();
        
        if c.is_digit(10) {
            // Build the current number
            current_number = current_number * 10 + c.to_digit(10).unwrap() as i64;
        } else {
            // If we hit an operator, we need to process the current number
            numbers.push(current_number);
            current_number = 0;

            if c == '(' {
                // Find the matching closing parenthesis
                let mut j = i;
                let mut balance = 1;
                while balance > 0 {
                    j += 1;
                    if j >= expr.len() {
                        panic!("Mismatched parentheses in expression");
                    }
                    if expr.chars().nth(j).unwrap() == '(' {
                        balance += 1;
                    } else if expr.chars().nth(j).unwrap() == ')' {
                        balance -= 1;
                    }
                }
                // Evaluate the expression inside the parentheses
                let sub_expr = &expr[i + 1..j];
                let sub_result = evaluate(sub_expr, memo);
                numbers.push(sub_result);
                i = j; // Move index to the closing parenthesis
            } else {
                // Process the operator
                operators.push(c);
            }
        }
        i += 1;
    }
    // Push the last number
    numbers.push(current_number);

    // Now we have all numbers and operators, we need to evaluate them
    max_value = calculate_max(&numbers, &operators);
    
    // Store the result in the memoization map
    memo.insert(expr.to_string(), max_value);
    max_value
}

fn calculate_max(numbers: &[i64], operators: &[char]) -> i64 {
    let mut max_value = i64::MIN;
    let n = numbers.len();

    // Generate all possible evaluations
    for i in 0..n {
        for j in (i + 1)..n {
            let left = evaluate_sub_expression(&numbers[i..=i], &operators[i..j]);
            let right = evaluate_sub_expression(&numbers[j..], &operators[j - 1..]);
            let result = apply_operator(left, right, operators[j - 1]);
            max_value = max_value.max(result);
        }
    }
    max_value
}

fn evaluate_sub_expression(numbers: &[i64], operators: &[char]) -> i64 {
    // This function evaluates a sub-expression based on the numbers and operators provided
    let mut result = numbers[0];
    for (i, &op) in operators.iter().enumerate() {
        let next_number = numbers[i + 1];
        result = apply_operator(result, next_number, op);
    }
    result
}

fn apply_operator(left: i64, right: i64, operator: char) -> i64 {
    match operator {
        '+' => left + right,
        '-' => left - right,
        '*' => left * right,
        _ => unreachable!(),
    }
}

fn main() {
    // Example usage
    let expression = "1+(2*3)-(4*5)";
    let result = max_expression_value(expression);
    println!("Max value: {}", result);
}
