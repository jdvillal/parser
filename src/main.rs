use std::{collections::HashMap, fmt, io::{self, Write}};
pub mod tests;

fn main() {
    let mut variables: HashMap<char, f32> = HashMap::new();
    loop{
        print!(">> ");
        io::stdout().flush().unwrap();
        let input = {
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
            buf
        };
        if input.trim() == "exit" {
            break;
        }
        let expr = Expression::from_str(&input);
        if let Some((var_name, lhs)) = expr.is_asign(){
            let value = lhs.eval(&variables);
            variables.insert(var_name, value);
            continue;
        }
        let value = expr.eval(&variables);
        println!("{}", value);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Atom(char),
    Op(char),
    Eof,
}

#[derive(Debug)]
struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(input: &str) -> Lexer {
        let mut tokens = input
            .chars()
            .filter(|it| !it.is_ascii_whitespace())
            .map(|c| match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' => Token::Atom(c),
                _ => Token::Op(c),
            })
            .collect::<Vec<_>>();
        tokens.reverse();
        Lexer { tokens }
    }

    fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    fn peek(&mut self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
}

#[derive(Clone)]
enum Expression {
    Atom(char),
    Operation(char, Vec<Expression>),
}
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Atom(i) => write!(f, "{}", i),
            Expression::Operation(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}

impl Expression {
    fn from_str(input: &str) -> Expression {
        let mut lexer = Lexer::new(input);
        parse_expression(&mut lexer, 0.0)
    }
    #[allow(unused)]
    fn is_asign(&self) -> Option<(char, &Expression)>{
        match self{
            Expression::Atom(_) => return None,
            Expression::Operation(c, operands) => {
                if *c == '=' {
                    let var_name = match operands.first().unwrap(){
                        Expression::Atom(c) => {
                            if *c >= 'a' && *c <= 'z' || *c >= 'A' && *c <= 'Z'  {
                                *c
                            }else {
                                panic!("Not a variable name: {}", c)
                            }
                        }
                        _ => unreachable!() 
                    };
                    return Some((var_name, operands.last().unwrap()));
                }
                return None;
            }
        }
    }
    #[allow(unused)]
    fn eval(&self, variables: &HashMap<char, f32>) -> f32{
        match self{
            Expression::Atom(c) => {
                    match c {
                        '0'..='9' => return c.to_digit(10).unwrap() as f32,
                        'a'..='z' | 'A'..='Z' => {
                            *variables.get(c).expect(&format!("Undefined variable {}", c))
                        },
                        _ => unreachable!()
                    }
            }
            Expression::Operation(operator, operands) => {
                let lhs = operands.first().unwrap().eval(variables);
                let rhs = operands.last().unwrap().eval(variables);
                match operator{
                    '+' => return lhs + rhs,
                    '-' => return lhs - rhs,
                    '*' => return lhs * rhs,
                    '/' => return lhs / rhs,
                    '^' => return lhs.powf(rhs),
                    '√' => return lhs.powf(1.0/(rhs)),
                    op => panic!("Bad operator: {}", op)
                }
            },
        }
    }
}

fn parse_expression(lexer: &mut Lexer, min_bp: f32) -> Expression {
    let mut lhs = match lexer.next() {
        Token::Atom(it) => Expression::Atom(it),
        Token::Op('(') => {
            let lhs = parse_expression(lexer, 0.0);
            assert_eq!(lexer.next(), Token::Op(')'));
            lhs
        }
        t => panic!("bad token: {:?}", t),
    };
    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Op(')') => break,
            Token::Op(op) => op,
            t => panic!("bad token: {:?}", t),
        };
        //My mistake: DO NOT call `lexer.next()` here
        let (l_bp, r_bp) = infix_binding_power(op);
        if l_bp < min_bp {
            break;
        }
        //In the video, `lexer.next()` is called BEFORE the if statement.
        //It must be called AFTER the precedence check, because calling it too early
        //would consume a token that shouldn't be parsed yet—leading to incorrect parse trees.
        lexer.next();
        let rhs = parse_expression(lexer, r_bp);
        lhs = Expression::Operation(op, vec![lhs, rhs]);
    }
    lhs
}

fn infix_binding_power(op: char) -> (f32, f32) {
    match op {
        '=' => (0.2, 0.1),
        '+' | '-' => (1.0, 1.1),
        '*' | '/' => (2.0, 2.1),
        '^' | '√' => (3.1, 3.0),
        '.' => (4.0, 4.1),
        _ => panic!("bad op: {:?}", op),
    }
}
