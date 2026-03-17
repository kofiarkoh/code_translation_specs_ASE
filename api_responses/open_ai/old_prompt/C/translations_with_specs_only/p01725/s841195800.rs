use std::cmp;

fn main() {
    let expression = "(1+2)*(3-4)"; // Example input; replace with actual input as needed
    let max_value = evaluate_expression(&expression);
    println!("{}", max_value);
}

fn evaluate_expression(expression: &str) -> i64 {
    // This function evaluates the expression and returns the maximum value
    let tokens = tokenize(expression);
    let mut max_value = i64::MIN;

    // Evaluate all possible operator precedence arrangements
    for i in 0..tokens.len() {
        if let Some(result) = evaluate(&tokens, 0, tokens.len() - 1) {
            max_value = cmp::max(max_value, result);
        }
    }

    max_value
}

fn tokenize(expression: &str) -> Vec<String> {
    // Tokenizes the input expression into numbers and operators
    let mut tokens = Vec::new();
    let mut current_number = String::new();

    for ch in expression.chars() {
        if ch.is_digit(10) {
            current_number.push(ch);
        } else {
            if !current_number.is_empty() {
                tokens.push(current_number.clone());
                current_number.clear();
            }
            tokens.push(ch.to_string());
        }
    }

    if !current_number.is_empty() {
        tokens.push(current_number);
    }

    tokens
}

fn evaluate(tokens: &[String], start: usize, end: usize) -> Option<i64> {
    // Evaluates the expression represented by tokens from start to end
    if start == end {
        return tokens[start].parse::<i64>().ok(); // Return the number as is
    }

    let mut max_result = i64::MIN;

    for i in (start..end).step_by(2) { // Operators are at odd indices
        let operator = &tokens[i];
        if let (Some(left), Some(right)) = (evaluate(tokens, start, i - 1), evaluate(tokens, i + 1, end)) {
            let result = match operator.as_str() {
                "+" => left + right,
                "-" => left - right,
                "*" => left * right,
                _ => continue, // Should not happen due to tokenization
            };
            max_result = cmp::max(max_result, result);
        }
    }

    if max_result == i64::MIN {
        None // No valid evaluation found
    } else {
        Some(max_result)
    }
}
