use std::io;

fn main() {
    println!("calc");
    loop {
        let mut prompt;
        loop {
            prompt = get_input("Enter your problem (or type 'exit' to quit): ");
            if prompt.trim().eq_ignore_ascii_case("exit") {
                return;
            }
            if validate_input(&prompt) {
                break;
            }
            println!("Invalid input. Please enter a valid mathematical expression.");
        }
        let problem = eval_with_oop(prompt.trim().to_string());
        println!("Result: {problem}");
    }
}

// Receive input as string
fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!();
    input
}

// Makes sure input includes no alphabet characters
fn validate_input(prompt: &str) -> bool {
    for c in prompt.chars() {
        if c.is_ascii_alphabetic() {
            return false;
        }
    }
    true
}
// lol look at func title
fn find_innermost_parentheses(problem: &str) -> Option<(usize, usize)> {
    let mut open_stack = Vec::new();
    
    for (i, c) in problem.chars().enumerate() {
        if c == '(' {
            open_stack.push(i);
        } else if c == ')' && !open_stack.is_empty() {
            let open_pos = open_stack.pop().unwrap();
            return Some((open_pos, i));
        }
    }
    None
}
// eval the problem from the parenthesis
fn eval_prob(problem: &str) -> String {
    let mut nums = Vec::new();  // Stack for numbers
    let mut ops: Vec<String> = Vec::new();   // Stack for operators

    let mut tokens = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = problem.chars().collect();
    
    while i < chars.len() {
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }
        
        if "+-*/^".contains(chars[i]) {
            tokens.push(chars[i].to_string());
            i += 1;
        } else if chars[i].is_digit(10) || chars[i] == '.' || (chars[i] == '-' && (i == 0 || !chars[i-1].is_digit(10))) {
            let mut num = String::new();
            if chars[i] == '-' {
                num.push(chars[i]);
                i += 1;
            }
            while i < chars.len() && (chars[i].is_digit(10) || chars[i] == '.') {
                num.push(chars[i]);
                i += 1;
            }
            tokens.push(num);
        } else {
            i += 1;
        }
    }
    
    for token in tokens {
        if let Ok(num) = token.parse::<f64>() {
            nums.push(num);
        } else if "+-*/^".contains(&token) {
            while !ops.is_empty() && precedence(ops.last().unwrap()) >= precedence(&token) {
                apply_operator(&mut nums, &mut ops);
            }
            ops.push(token);
        } else {
            return format!("Error: Invalid token '{}'", token);
        }
    }

    while !ops.is_empty() {
        apply_operator(&mut nums, &mut ops);
    }

    if nums.is_empty() {
        return "Error: No result".to_string();
    }
    
    nums.pop().unwrap().to_string()
}

// defines prec
fn precedence(op: &str) -> i32 {
    match op {
        "+" | "-" => 1,
        "*" | "/" => 2,
        "^"       => 3,
        _ => 0,
    }
}

// applies opp to the problem
fn apply_operator(nums: &mut Vec<f64>, ops: &mut Vec<String>) {
    if nums.len() < 2 { return; }

    let b = nums.pop().unwrap();
    let a = nums.pop().unwrap();
    let op = ops.pop().unwrap();

    let result = match op.as_str() {
        "^" => f64::powf(a, b),
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => {
            if b == 0.0 {
                f64::NAN  // Handle division by zero
            } else {
                a / b
            }
        },
        _ => unreachable!(),
    };

    nums.push(result);
}
//overall home ish func
fn eval_with_oop(mut expr: String) -> String {
    while let Some((start, end)) = find_innermost_parentheses(&expr) {
        let inner_expr = &expr[start + 1..end];
        let result = eval_prob(inner_expr);
        
        if result.starts_with("Error") {
            return result;
        }
        
        expr.replace_range(start..=end, &result);
    }
    
    eval_prob(&expr)
}
