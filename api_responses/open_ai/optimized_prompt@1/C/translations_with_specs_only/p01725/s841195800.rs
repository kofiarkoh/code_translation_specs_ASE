use std::collections::HashMap;

pub fn min_expression_value(expression: &str) -> i64 {
    // This function evaluates the expression and returns the minimum value
    // by considering all possible operator precedence configurations.
    
    // Parse the expression into tokens (numbers and operators)
    let tokens = tokenize(expression);
    
    // Evaluate the expression with different operator precedence
    let mut results = HashMap::new();
    evaluate(&tokens, &mut results);
    
    // Return the minimum result found
    *results.values().min().unwrap()
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
            if "+-*/()".contains(ch) {
                tokens.push(ch.to_string());
            }
        }
    }
    
    if !current_number.is_empty() {
        tokens.push(current_number);
    }
    
    tokens
}

fn evaluate(tokens: &[String], results: &mut HashMap<String, i64>) {
    // Evaluates the expression represented by tokens and stores results
    // for different operator precedence configurations.
    
    // Base case: if only one number is present, return its value
    if tokens.len() == 1 {
        let value = tokens[0].parse::<i64>().unwrap();
        results.insert(tokens[0].clone(), value);
        return;
    }
    
    for i in 0..tokens.len() {
        if is_operator(&tokens[i]) {
            let left_tokens = &tokens[0..i];
            let right_tokens = &tokens[i + 1..];
            
            // Evaluate left and right parts recursively
            let left_results = evaluate_part(left_tokens);
            let right_results = evaluate_part(right_tokens);
            
            // Combine results based on the operator
            for &left in &left_results {
                for &right in &right_results {
                    let result = match tokens[i].as_str() {
                        "+" => left + right,
                        "-" => left - right,
                        "*" => left * right,
                        _ => unreachable!(),
                    };
                    results.insert(format!("{}{}{}", left, tokens[i], right), result);
                }
            }
        }
    }
}

fn evaluate_part(tokens: &[String]) -> Vec<i64> {
    // Evaluates a part of the expression and returns possible results
    let mut results = Vec::new();
    
    if tokens.len() == 1 {
        let value = tokens[0].parse::<i64>().unwrap();
        results.push(value);
    } else {
        for i in 0..tokens.len() {
            if is_operator(&tokens[i]) {
                let left_tokens = &tokens[0..i];
                let right_tokens = &tokens[i + 1..];
                
                let left_results = evaluate_part(left_tokens);
                let right_results = evaluate_part(right_tokens);
                
                for &left in &left_results {
                    for &right in &right_results {
                        let result = match tokens[i].as_str() {
                            "+" => left + right,
                            "-" => left - right,
                            "*" => left * right,
                            _ => unreachable!(),
                        };
                        results.push(result);
                    }
                }
            }
        }
    }
    
    results
}

fn is_operator(token: &str) -> bool {
    // Checks if the token is an operator
    matches!(token.as_str(), "+" | "-" | "*")
}

// Example usage
fn main() {
    let expression = "2*(3+4)";
    let min_value = min_expression_value(expression);
    println!("Minimum value: {}", min_value);
}
