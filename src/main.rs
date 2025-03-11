use std::io;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

#[derive(Debug)]
enum CalcError {
    InvalidInput,
    DivisionByZero,
    UnbalancedParentheses,
}

fn precedence(op: &str) -> i32 {
    match op {
        "sin" => 5,
        "sqrt" => 4,
        "^" => 3,
        "*" | "/" => 2,
        "+" | "-" => 1,
        _ => 0,
    }
}

fn apply_operator(a: f64, b: f64, op: &str) -> Result<f64, CalcError> {
    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err(CalcError::DivisionByZero) 
            } else {
                Ok(a / b)
            }
        }
        "^" => Ok(a.powf(b)),
        _ => Err(CalcError::InvalidInput),
    }
}

fn rpn(expr: &str) -> Result<VecDeque<String>, CalcError> {
    let mut output: VecDeque<String> = VecDeque::new();
    let mut operators = Vec::new();
    let tokens = tokenize(expr)?;

    for token in tokens {
        if let Ok(_) = f64::from_str(&token) {
            output.push_back(token);
        } else if token == "(" {
            operators.push(token);
        } else if token == ")" {
            while let Some(op) = operators.pop() {
                if op == "(" {
                    break;
                }
                output.push_back(op);
            }
        } else {
            while let Some(op) = operators.last() {
                if precedence(op) >= precedence(&token) {
                    output.push_back(operators.pop().unwrap());
                } else {
                    break;
                }
            }
            operators.push(token);
        }
    }
    while let Some(op) = operators.pop() {
        if op == "(" || op == ")" {
            return Err(CalcError::UnbalancedParentheses);
        }
        output.push_back(op);
    }
    Ok(output)
}

fn eval_pn(tokens: VecDeque<String>) -> Result<f64, CalcError> {
    let mut stack = Vec::new();

    for token in tokens {
        if let Ok(num) = f64::from_str(&token) {
            stack.push(num);
        } else if token == "√" || token == "sqrt" {
            let a = stack.pop().ok_or(CalcError::InvalidInput)?; // Prevent panic
            if a < 0.0 {
                return Err(CalcError::InvalidInput);
            }
            stack.push(a.sqrt());
        } else if token == "sin" {
            let a = stack.pop().ok_or(CalcError::InvalidInput)?; // Prevent panic
            stack.push(a.sin());
        } 
        else {
            if stack.len() < 2 {
                return Err(CalcError::InvalidInput);
            }
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            stack.push(apply_operator(a, b, &token)?);
        }
    }

    // Ensure exactly one result remains
    if stack.len() == 1 {
        stack.pop().ok_or(CalcError::InvalidInput)
    } else {
        Err(CalcError::InvalidInput)
    }
}


fn tokenize(expr: &str) -> Result<Vec<String>, CalcError> {
    let allowed_chars: HashSet<char> = "0123456789.+-*/^()√ abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut tokens = Vec::new();
    let mut num = String::new();
    let mut i = 0;
    let chars: Vec<char> = expr.chars().collect();

    while i < chars.len() {
        let c = chars[i];

        if !allowed_chars.contains(&c) {
            return Err(CalcError::InvalidInput);
        }

        if c == 's' && i + 3 < chars.len() && &chars[i..i+4] == ['s', 'q', 'r', 't'] {
            if !num.is_empty() {
                tokens.push(num.clone());
                num.clear();
            }
            tokens.push("sqrt".to_string());
            i += 4;
            continue;
        }if c == 's' && i + 2 < chars.len() && &chars[i..i+4] == ['s', 'i', 'n'] {
            if !num.is_empty() {
                tokens.push(num.clone());
                num.clear();
            }
            tokens.push("sin".to_string());
            i += 3;
            continue;
        }

        if c.is_digit(10) || c == '.' {
            num.push(c);
        } else {
            if !num.is_empty() {
                tokens.push(num.clone());
                num.clear();
            }
            if c != ' ' {
                tokens.push(c.to_string());
            }
        }
        i += 1;
    }

    if !num.is_empty() {
        tokens.push(num);
    }
    println!("{:?}", tokens);
    Ok(tokens)
}

fn eval_expression(expr: &str) -> Result<f64, CalcError> {
    let pf = rpn(expr)?;
    eval_pn(pf)
}

fn main() {
    println!("calc");
    loop {
        let input = get_input("Enter an expression (or 'exit' to quit): ");
        if input.trim().eq_ignore_ascii_case("exit") {
            break;
        }
        match eval_expression(&input) {
            Ok(result) => println!("Result: {}", result),
            Err(err) => println!("Error: {:?}", err),
        }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}