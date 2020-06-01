use std::result::Result;

fn main() {
    loop {
        // Read input from the user and just do nothing when the input is empty
        let input = read_string();
        if input.is_empty() {
            continue;
        }
        let in_vec = tokenize(&input);
        
        for elem in in_vec {
            print!("{:?}, ", elem); 
        }

        // Debug output
        //println!("{}", input);
    }
}


/// Reads a string from the user (with a nice prompt).
fn read_string() -> String {
    use std::io::Write;

    // Print prompt
    print!("calc > ");
    std::io::stdout().flush().unwrap();

    // Read line
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("something went horribly wrong...");

    // Discard trailing newline
    let new_len = buffer.trim_right().len();
    buffer.truncate(new_len);

    buffer
}

// Mathematical "errors" we want to catch
#[derive(Debug)]
#[derive(PartialEq)]
pub enum ParseError {
    InvalidCharacter,
}

//
fn tokenize(input: &str)->Result<Vec<Token>, ParseError>{
    use Token::*;

    let mut tkn_vec = Vec::new();
    for c in input.chars() {
        let token = match c {
            '+' => Plus,
            '-' => Minus,
            '*' => Asterisk,
            '/' => Slash,
            '(' => OpenParen,
            ')' => CloseParen,
            chr if chr.is_whitespace() => continue,
            chr if chr.is_digit(10) => { 
                //we ask is_digit, and then convert -> if error, panic!
                let res = chr.to_digit(10).unwrap() as usize;
                Number(res) 
            },
            _ => { return Err(ParseError::InvalidCharacter); }, 
        };
        tkn_vec.push(token);
    }
    Ok(tkn_vec)
}

/// Token to hold one element for the calculator
/// So this can be a number, a operand, a paranthese
#[derive(PartialEq)]
#[derive(Debug)]
enum Token {
    Plus,
    Minus,
    Asterisk,
    Slash,
    OpenParen,
    CloseParen,
    Number(usize),
}

type ExprParsRes = Result<Expr, ParseError>;

enum Expr {
    Leaf(isize),
    Internal {
        children: Vec<Expr>,
        op: Op,
    },
}

/*enum ParseState {
    ExpectOperand,
    ExpectOperator,
}*/

impl Expr {
    // parse input like: 3 + (8*7) + ((5/8)+6)
    fn parse(input: &Vec<Token>, idx: &mut usize) -> ExprParsRes{

        //1. Operand
        let mut expect_closing_paranthese = false;
        let child1_res = if input[*idx] == Token::OpenParen {
            *idx += 1;
            expect_closing_paranthese = true;
            Expr::parse(input, idx)
            //Call another parse method starting from here
        }else {
            //this better be a number, or we have a problem
            match input[*idx] { 
                Token::Number(nr) => {
                    *idx += 1;
                    Ok(Expr::Leaf(nr as isize))
                },
                _ => unimplemented!(),  //TODO replace by proper Error message 
            }
        };

        let child1 = match child1_res {
            Ok(child) => child,
            Err(err) => { return Err(err); },
        };
        
        if expect_closing_paranthese {
            assert_eq!(input[*idx], Token::CloseParen);
            *idx += 1;
        }

        //if this was all, let's end
        if *idx == input.len() {
            return Ok(child1);
        }

        //2. Operator
        let op = match input[*idx] {
                Token::Plus => Op::Addition,
                Token::Minus => Op::Subtraction,
                Token::Asterisk => Op::Multiplication,
                Token::Slash => Op::Division,
                _ => unimplemented!(),
        };
        *idx += 1;

        //if this was all, we're missing the second operand
        if *idx == input.len() {
            return Err(ParseError::InvalidCharacter);
        }
        
        //3. Operand
        expect_closing_paranthese = false;
        let child2_res = if input[*idx] == Token::OpenParen {
            *idx += 1;
            expect_closing_paranthese = true;
            Expr::parse(input, idx)
            //Call another parse method starting from here
        }else {
            //this better be a number, or we have a problem
            match input[*idx] { 
                Token::Number(nr) => {
                    *idx += 1;
                    Ok(Expr::Leaf(nr as isize))
                },
                _ => unimplemented!(),  //TODO replace by proper Error message 
            }
        };

        let child2 = match child2_res {
            Ok(child) => child,
            Err(err) => { return Err(err); },
        };

        if expect_closing_paranthese {
            assert_eq!(input[*idx], Token::CloseParen);
            *idx += 1;
        }

        //if this was all, let's end
        let tree = Expr::Internal {
            children: vec![child1, child2],
            op: op,
        };
        return Ok(tree);
    }

    fn evaluate(&self) -> isize {
        match *self {
            Expr::Leaf(a) => a,
            Expr::Internal{ref children, ref op} => {
                op.apply(children[0].evaluate(), children[1].evaluate())
            }
        }
    }
}

enum Op {
   Addition,
   Subtraction,
   Multiplication,
   Division,
}

impl Op {
    fn apply(&self, a: isize, b: isize) -> isize{
        match *self {
            Op::Addition =>  a + b,
            Op::Subtraction =>  a - b,
            Op::Multiplication =>  a * b,
            Op::Division =>  a / b,
        }
    }
}

/// Section of Tests for tokenize

#[test]
fn test_general_stuff() {
    let string = "1+2+3-4*(6+7)".to_string();
    let str_vec = vec![Token::Number(1),
                       Token::Plus,
                       Token::Number(2),
                       Token::Plus,
                       Token::Number(3),
                       Token::Minus,
                       Token::Number(4),
                       Token::Asterisk,
                       Token::OpenParen,
                       Token::Number(6),
                       Token::Plus,
                       Token::Number(7),
                       Token::CloseParen,];

    match tokenize(&string) {
        Ok(res) => assert_eq!(res, str_vec),
        Err(e) => panic!("Some Error: {:?}", e),
    }
}

#[test]
fn test_ignores_whitespace() {
    let string = "1 + 2   +  3 -\t4 * (  6  + 7 )\n".to_string();
    let str_vec = vec![Token::Number(1),
                       Token::Plus,
                       Token::Number(2),
                       Token::Plus,
                       Token::Number(3),
                       Token::Minus,
                       Token::Number(4),
                       Token::Asterisk,
                       Token::OpenParen,
                       Token::Number(6),
                       Token::Plus,
                       Token::Number(7),
                       Token::CloseParen,];

    match tokenize(&string) {
        Ok(res) => assert_eq!(res, str_vec),
        Err(e) => panic!("Some Error: {:?}", e),
    }
}

#[test]
fn test_ignores_non_ascii() {
    let string = "1 + 2ê +©3-4 * (∑  6  + 7æ)".to_string();

    match tokenize(&string) {
        Ok(res) => panic!("Method tokenize() returned {:?}, instead of Err() type", res),
        Err(e) => assert_eq!(e, ParseError::InvalidCharacter),
    }
}

#[test]
fn test_empty_expression() {
    //Generate 0 + 0
    let a = Expr::Leaf(0);
    let b = Expr::Leaf(0);
    let tree = Expr::Internal 
        { 
            children: vec![a, b], 
            op: Op::Addition,
        };
    assert_eq!(tree.evaluate(), 0);
}

#[test]
fn test_simple_expression() {
    //Generate 3 + 4
    let a = Expr::Leaf(3);
    let b = Expr::Leaf(4);
    let tree = Expr::Internal
        {
            children: vec![a, b],
            op: Op::Addition,
        };
    assert_eq!(tree.evaluate(), 7); 
}

#[test]
fn test_two_layer_expression() {
    // Test (3+5)*17
    let a = Expr::Leaf(3);
    let b = Expr::Leaf(5);
    let subtree = Expr::Internal
        {
            children: vec![a, b],
            op: Op::Addition,
        };
    
    let c = Expr::Leaf(17);
    let tree = Expr::Internal 
        {
            children: vec![subtree, c],
            op: Op::Multiplication,
        };
    assert_eq!(tree.evaluate(), 136);
}

#[test]
fn test_three_layer_expression() {
    // Test ((10/2)+4)*(6-9)
    let a = Expr::Leaf(10);
    let b = Expr::Leaf(2);
    let c = Expr::Leaf(4);
    let d = Expr::Leaf(6);
    let e = Expr::Leaf(9);

    let divtree = Expr::Internal 
        {
            children: vec![a, b],
            op: Op::Division,
        };
    let addtree = Expr::Internal
        {
            children: vec![divtree, c],
            op: Op::Addition,
        };
    let subtree = Expr::Internal
    {
        children: vec![d, e],
        op: Op::Subtraction,
    };
    let tree = Expr::Internal
    {
        children: vec![addtree, subtree],
        op: Op::Multiplication,
    };
    assert_eq!(tree.evaluate(), -27);
}

#[test]
fn test_parse_single_digit() {
    let tkn_vec = vec![Token::Number(1)];
    let mut idx = 0;
    let expr_res = Expr::parse(&tkn_vec, &mut idx);
    match expr_res {
        Ok(expr) => { 
            assert_eq!(expr.evaluate(), 1);
            println!("1 evaluated to {}", expr.evaluate());
        },
        Err(_) => panic!(), 
    }
}

#[test]
fn test_parse_simple_addition() {
    let tkn_vec = vec![
        Token::Number(3),
        Token::Plus,
        Token::Number(9),
    ];
    let mut idx = 0;
    let expr_res = Expr::parse(&tkn_vec, &mut idx);
    match expr_res {
        Ok(expr) => assert_eq!(expr.evaluate(), 12),
        Err(err) => { println!("Found err {:?}", err); panic!() },
    }
}

#[test]
fn test_parse_simple_addition_in_paranthese() {
    let tkn_vec = vec![
        Token::OpenParen,
        Token::Number(3),
        Token::Plus,
        Token::Number(9),
        Token::CloseParen,
    ];
    let mut idx = 0;
    let expr_res = Expr::parse(&tkn_vec, &mut idx);
    match expr_res {
        Ok(expr) => assert_eq!(expr.evaluate(), 12),
        Err(err) => { println!("Found err {:?}", err); panic!() },
    }
}

#[test]
fn test_parse_complex_addition_with_parantheses() {
 
    let string = "(1+2)+(3+(6+9))".to_string();

    let tkn_vec = match tokenize(&string) {
        Ok(res) => res,
        Err(e) => panic!("Tokenize Error: {:?}", e),
    };
    let mut idx = 0;
    let expr_res = Expr::parse(&tkn_vec, &mut idx);
    match expr_res {
        Ok(expr) => assert_eq!(expr.evaluate(), 21),
        Err(err) => { println!("Found err {:?}", err); panic!() },
    }
}

#[test]
fn test_parse_complex_subtraction_with_parantheses() {
 
    let string = "(2-1)-(3+(6-9))".to_string();

    let tkn_vec = match tokenize(&string) {
        Ok(res) => res,
        Err(e) => panic!("Tokenize Error: {:?}", e),
    };
    let mut idx = 0;
    let expr_res = Expr::parse(&tkn_vec, &mut idx);
    match expr_res {
        Ok(expr) => assert_eq!(expr.evaluate(), 1),
        Err(err) => { println!("Found err {:?}", err); panic!() },
    }
}
