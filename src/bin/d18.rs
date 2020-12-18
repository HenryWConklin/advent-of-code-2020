use std::io::{stdin, BufRead};

use itertools::Itertools;

enum Operations {
    None,
    Mul,
    Add,
}

fn parse_expression(s: &str, i: &mut usize) -> u64 {
    let mut acc = 0;
    let mut op = Operations::None;
    while *i < s.len() {
        let val = match s.chars().nth(*i).unwrap() {
            ' ' => None,
            '0'..='9' => Some(s[*i..*i + 1].parse().unwrap()),
            '(' => {
                *i += 1;
                Some(parse_expression(s, i))
            }
            ')' => {
                return acc;
            }
            '+' => {
                op = Operations::Add;
                None
            }
            '*' => {
                op = Operations::Mul;
                None
            }
            _ => panic!("Invalid character"),
        };
        if let Some(v) = val {
            match op {
                Operations::None => acc = v,
                Operations::Add => acc += v,
                Operations::Mul => acc *= v,
            }
        }
        *i += 1;
    }
    acc
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Token {
    LeftParen,
    RightParen,
    Add,
    Mul,
    Value(u64),
}

fn tokenize(s: &str) -> Vec<Token> {
    let mut res = Vec::new();
    for c in s.chars() {
        let v = match c {
            '0'..='9' => Some(Token::Value(c.to_digit(10).unwrap() as u64)),
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            '+' => Some(Token::Add),
            '*' => Some(Token::Mul),
            _ => None,
        };
        if let Some(t) = v {
            res.push(t)
        }
    }
    res
}

fn parse2(s: &mut Vec<Token>, start: usize, mut end: usize) {
    // Parens
    while let Some((ind, _)) = s[start..end]
        .iter()
        .find_position(|&&t| t == Token::LeftParen)
    {
        let mut par_count = 1;
        let mut end_ind = ind + 1;
        // Find the index of the matching closing parentheses
        while par_count > 0 {
            match s[start + end_ind] {
                Token::LeftParen => par_count += 1,
                Token::RightParen => par_count -= 1,
                _ => {}
            }
            end_ind += 1;
        }
        // Recurse
        parse2(s, start + ind + 1, start + end_ind - 1);

        // Remove the surrounding parentheses,
        s.remove(start + ind);
        s.remove(start + ind + 1);
        // Adjust end index for the tokens removed in the recursive call and the removed parens
        end -= end_ind - ind - 1;
    }
    // Adds
    while let Some((ind, _)) = s[start..end].iter().find_position(|&&t| t == Token::Add) {
        let ind = start + ind;
        let val = match s[ind - 1..=ind + 1] {
            [Token::Value(v1), Token::Add, Token::Value(v2)] => v1 + v2,
            _ => panic!("Invalid add"),
        };
        s[ind] = Token::Value(val);
        s.remove(ind - 1);
        s.remove(ind);
        end -= 2;
    }
    // Multiplies
    while let Some((ind, _)) = s[start..end].iter().find_position(|&&t| t == Token::Mul) {
        let ind = start + ind;
        let val = match s[ind - 1..=ind + 1] {
            [Token::Value(v1), Token::Mul, Token::Value(v2)] => v1 * v2,
            _ => panic!("Invalid mul"),
        };
        s[ind] = Token::Value(val);
        s.remove(ind - 1);
        s.remove(ind);
        end -= 2;
    }
}

fn main() {
    let lines: Vec<_> = stdin().lock().lines().map(|l| l.unwrap()).collect();

    let part1: u64 = lines.iter().map(|s| parse_expression(s, &mut 0)).sum();
    println!("{}", part1);

    let part2: u64 = lines
        .iter()
        .map(|l| {
            let mut tokens = tokenize(l);
            let ntokens = tokens.len();
            parse2(&mut tokens, 0, ntokens);
            match tokens.as_slice() {
                [Token::Value(v)] => *v,
                _ => panic!("Failed to parse"),
            }
        })
        .sum();
    println!("{}", part2);
}
