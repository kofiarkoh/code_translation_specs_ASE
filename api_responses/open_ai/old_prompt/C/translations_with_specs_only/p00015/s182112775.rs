fn main() {
    // Example input for testing
    let datasets = vec![
        ("1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890", 
         "9876543210987654321098765432109876543210987654321098765432109876543210987654321098765432109876543210"),
        ("1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890", 
         "1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890"),
    ];

    for (first_num, second_num) in datasets {
        process_numbers(first_num, second_num);
    }
}

fn process_numbers(first_num: &str, second_num: &str) {
    // Check for overflow based on length
    if first_num.len() > 80 || second_num.len() > 80 {
        println!("overflow");
        return;
    }

    // Convert strings to integer arrays
    let f_num = convert_to_int_array(first_num);
    let s_num = convert_to_int_array(second_num);
    
    // Calculate the sum
    let sum = add_arrays(&f_num, &s_num);
    
    // Print the result or overflow message
    if sum.is_none() {
        println!("overflow");
    } else {
        println!("{}", sum.unwrap());
    }
}

fn convert_to_int_array(num_str: &str) -> Vec<u8> {
    // Convert each character to its corresponding integer value
    num_str.chars().rev().map(|c| c.to_digit(10).unwrap() as u8).collect()
}

fn add_arrays(f_num: &[u8], s_num: &[u8]) -> Option<String> {
    let mut carry = 0;
    let mut result = Vec::new();
    let max_len = f_num.len().max(s_num.len());

    for i in 0..max_len {
        let f_digit = *f_num.get(i).unwrap_or(&0);
        let s_digit = *s_num.get(i).unwrap_or(&0);
        
        let sum = f_digit + s_digit + carry;
        if sum > 9 {
            carry = 1;
            result.push(sum % 10);
        } else {
            carry = 0;
            result.push(sum);
        }
    }

    // If there's a carry left after the last addition
    if carry > 0 {
        if result.len() >= 80 { // Check for overflow
            return None;
        }
        result.push(carry);
    }

    // Convert result back to string
    result.reverse();
    Some(result.iter().map(|&d| (d + b'0') as char).collect())
}
