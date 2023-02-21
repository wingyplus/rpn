use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let result = compile_and_eval(input);
    println!("{:?}", result);
}

fn compile_and_eval(input: String) -> Result<i64, ()> {
    let tokens = compile(input).unwrap();
    eval(&tokens)
}

fn eval(tokens: &Vec<Token>) -> Result<i64, ()> {
    let mut stack: Vec<i64> = Vec::new();
    for token in tokens {
        match token {
            Token::Number(n) => stack.push(Into::<i64>::into(*n)),
            Token::Plus => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            },
            Token::Minus => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a - b);
            },
            Token::Multiply => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a * b);
            },
            Token::Divide => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a / b);
            },
            _ => {}
        }
    }
    Ok(stack.pop().unwrap())
}

#[derive(Debug)]
struct CompilationError(char);

fn compile(source: String) -> Result<Vec<Token>, CompilationError> {
    let tokens = scan(source);
    for token in &tokens {
        match token {
            Token::Invalid(ch) => return Err(CompilationError(*ch)),
            _ => {}
        }
    }
    Ok(tokens)
}

#[derive(Debug)]
enum Token {
    Number(u32),
    Plus,
    Minus,
    Multiply,
    Divide,
    Invalid(char),
}

// Too many bugs.
fn scan(source: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    for ch in source.chars() {
        match ch {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Multiply),
            '/' => tokens.push(Token::Divide),
            ' ' | '\n' | '\t' => {} // skip
            ch if ch.is_digit(10) => tokens.push(Token::Number(ch.to_digit(10).unwrap())),
            ch => tokens.push(Token::Invalid(ch)),
        }
    }
    tokens
}
