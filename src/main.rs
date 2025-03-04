use std::io;
use std::collections::HashSet;

// Enum for more robust error handling
#[derive(Debug)]
enum CalcError {
    InvalidInput,
    DivisionByZero,
    ParseError,
    SqrtNegative,
    UnbalancedParentheses,
}

// Improved input validation
fn validate_input(input: &str) -> Result<(), CalcError> {
    // Allowed characters set
    let allowed_chars: HashSet<char> = 
        "0123456789.+-*/^() ".chars().collect(); //allowed chars, all math and letters
    
    let mut paren_count = 0; // balances parenthesis by adding for opening and lowering for a closing one
    for c in input.chars() {
        if !allowed_chars.contains(&c) {
            return Err(CalcError::InvalidInput);
        }
        
        if c == '(' {
            paren_count += 1;
        } else if c == ')' {
            paren_count -= 1;
        }
        
        if paren_count < 0 {
            return Err(CalcError::UnbalancedParentheses);
        }
    }
    
    if paren_count != 0 {
        return Err(CalcError::UnbalancedParentheses);
    }
    
    Ok(())
}

// More robust error handling for parsing
fn parse_number(token: &str) -> Result<f64, CalcError> {
    token.parse::<f64>().map_err(|_| CalcError::ParseError)
}

// Improved operator precedence and handling
fn apply_operator(nums: &mut Vec<f64>, ops: &mut Vec<String>) -> Result<(), CalcError> {
    if nums.is_empty() || ops.is_empty() {
        return Err(CalcError::InvalidInput);
    }

    let op = ops.pop().unwrap();
    match op.as_str() {
        "sqrt" => {
            let a = nums.pop().ok_or(CalcError::InvalidInput)?;
            if a < 0.0 {
                return Err(CalcError::SqrtNegative);
            }
            nums.push(a.sqrt());
        },
        "+" | "-" | "*" | "/" | "^" => {
            if nums.len() < 2 {
                return Err(CalcError::InvalidInput);
            }
            
            let b = nums.pop().unwrap();
            let a = nums.pop().unwrap();
            
            let result = match op.as_str() {
                "^" => f64::powf(a, b),
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => {
                    if b == 0.0 {
                        return Err(CalcError::DivisionByZero);
                    }
                    a / b
                },
                _ => unreachable!(),
            };

            nums.push(result);
        },
        _ => return Err(CalcError::InvalidInput),
    }
    
    Ok(())
}

// expression validation
fn eval_expression(expr: &str) -> Result<f64, CalcError> {
    
    validate_input(expr)?;

    let mut nums: Vec<f64> = Vec::new();
    let mut ops: Vec<String> = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = expr.chars().collect();

    while i < chars.len() {
       
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }

        //if char is a digit or decimal point or minus 
        if chars[i].is_digit(10) || chars[i] == '.' || 
           (chars[i] == '-' && (i == 0 || "+-*/^(".contains(chars[i-1]))) {
            let mut num_str = String::new();
            //if negative, num is neg
            if chars[i] == '-' {
                num_str.push('-');
                i += 1;
            }

            //parse digits and decimals
            while i < chars.len() && (chars[i].is_digit(10) || chars[i] == '.') {
                num_str.push(chars[i]);
                i += 1;
            }

            let num = parse_number(&num_str)?;
            nums.push(num);
        }
        // if op, push the opp
        else if "+-*/^".contains(chars[i]) {
            ops.push(chars[i].to_string());
            i += 1;
        }
        // handle sqrt
        else if i + 3 < chars.len() && &expr[i..i+4] == "sqrt" {
            ops.push("sqrt".to_string());
            i += 4;
        }
        else {
            return Err(CalcError::InvalidInput);
        }
    }

    
    while !ops.is_empty() {
        apply_operator(&mut nums, &mut ops)?;
    }

    // return result
    nums.first()
        .cloned()
        .ok_or(CalcError::InvalidInput)
}

// Main function with improved error handling
fn main() {
    println!("Advanced Calculator");
    loop {
        let input = get_input("Enter an expression (or 'exit' to quit): ");
        
        if input.trim().eq_ignore_ascii_case("exit") { 
            break;
        }

        match eval_expression(&input) { // error net, i lowkey dk how this works
            Ok(result) => println!("Result: {}", result),
            Err(err) => match err {
                CalcError::InvalidInput => 
                    println!("Invalid input. Please check your expression."),
                CalcError::DivisionByZero => 
                    println!("Error: Division by zero is not allowed."),
                CalcError::ParseError => 
                    println!("Error: Could not parse the number."),
                CalcError::SqrtNegative => 
                    println!("Error: Cannot calculate square root of a negative number."),
                CalcError::UnbalancedParentheses => 
                    println!("Error: Unbalanced parentheses in the expression."),
            }
        }
    }
}

// Input function remains mostly the same
fn get_input(prompt: &str) -> String {

    println!("{}", prompt);
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}