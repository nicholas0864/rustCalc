use std::io;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

#[derive(Debug)]
enum CalcError {
    InvalidInput,
    DivisionByZero,
    ParseError,
    SqrtNegative,
    UnbalancedParentheses,
}

fn precedence(op: &str) -> i32 {
    match op {
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
        } else {
            if stack.len() < 2 {
                return Err(CalcError::InvalidInput);
            }
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            stack.push(apply_operator(a, b, &token)?);
        }
    }
    stack.pop().ok_or(CalcError::InvalidInput)
}

fn tokenize(expr: &str) -> Result<Vec<String>, CalcError> {
    let allowed_chars: HashSet<char> = "0123456789.+-*/^() ".chars().collect();
    let mut tokens = Vec::new();
    let mut num = String::new();

    for c in expr.chars() {
        if !allowed_chars.contains(&c) {
            return Err(CalcError::InvalidInput);
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
    }
    if !num.is_empty() {
        tokens.push(num);
    }
    Ok(tokens)
}

fn eval_expression(expr: &str) -> Result<f64, CalcError> {
    let postfix = rpn(expr)?;
    eval_pn(postfix)
}

fn main() {
    println!("Advanced Calculator");
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
