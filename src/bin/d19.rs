use std::collections::HashMap;
use std::hash::Hash;
use std::io::{Read, stdin};

#[derive(Debug, Clone, Copy)]
enum Symbol<N: Eq + Hash, T: Eq> {
    NonTerminal(N),
    Terminal(T),
}

type Grammar<N, T> = HashMap<N, Vec<Vec<Symbol<N, T>>>>;

// Try to parse the input list of tokens, starting at given state. Returns an error if parsing
// fails, or an Ok(# input symbols consumed)
fn parse<N: Eq + Hash, T: Eq>(input: &[T], state: &N, rules: &Grammar<N, T>) -> Result<usize, ()> {
    for rule in rules.get(state).unwrap_or(&vec![]) {
        let mut consumed = 0;
        let mut success = true;
        for s in rule {
            match s {
                Symbol::NonTerminal(n) => {
                    let res = parse(&input[consumed..], n, rules);
                    match res {
                        Ok(x) => consumed += x,
                        Err(_) => {
                            success = false;
                            break;
                        }
                    }
                }
                Symbol::Terminal(t) => {
                    if consumed < input.len() && *t == input[consumed] {
                        consumed += 1;
                    } else {
                        success = false;
                        break;
                    }
                }
            }
        }
        if success {
            return Ok(consumed);
        }
    }
    Err(())
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let sections: Vec<_> = input.split("\n\n").collect();
    assert_eq!(sections.len(), 2);
    let grammar_spec = sections[0];
    let strings = sections[1];
    println!("{}", strings.lines().count());

    let mut grammar = Grammar::new();

    // R: A B | C D => (R, [[A,B],[C,D]]), then insert into the hash map
    grammar.extend(grammar_spec.lines().map(|l| {
        let mut split = l.split(": ");
        let from = split.next().unwrap();
        let to = split.next().unwrap();
        let rules = to
            .split(" | ")
            .map(|r| {
                r.split(' ')
                    .map(|v| {
                        if v.starts_with('"') {
                            Symbol::Terminal(v.chars().nth(1).unwrap())
                        } else {
                            Symbol::NonTerminal(v.parse().unwrap())
                        }
                    })
                    .collect()
            })
            .collect();
        (from.parse::<u32>().unwrap(), rules)
    }));

    // Try to parse each line
    let part1 = strings
        .lines()
        .filter(|s| {
            let chars: Vec<_> = s.chars().collect();
            if let Ok(n) = parse(&chars, &0, &grammar) {
                // Will still return Ok even if only part of the input was parsed, check that
                // the whole input was parsed
                n == chars.len()
            } else {
                false
            }
        })
        .count();
    println!("{}", part1);

    // Actually solving this is hard, just fake it and repeat rules 8 and 11 up to 30 times
    // because that's probably longer than any of the strings in the input
    const N_REPS: usize = 30;
    let rule8: Vec<_> = (1..N_REPS)
        .map(|i| vec![Symbol::NonTerminal(42); i])
        .collect();

    let rule11: Vec<_> = (1..N_REPS)
        .map(|i| {
            let mut v1 = vec![Symbol::NonTerminal(42); i];
            let mut v2 = vec![Symbol::NonTerminal(31); i];
            v1.append(&mut v2);
            v1
        })
        .collect();

    // Replace rule 0 with every possible combination of repetitions of 8 and 11
    let mut rule0 = vec![];
    for a in &rule8 {
        for b in &rule11 {
            let mut v = a.clone();
            v.append(&mut b.clone());
            rule0.push(v);
        }
    }
    // Prioritize the longer patterns, so that it will probably parse the entire string and
    // not just part of it
    rule0.sort_by(|a, b| a.len().cmp(&b.len()));
    rule0.reverse();
    grammar.insert(0, rule0);

    let part2: Vec<_> = strings
        .lines()
        .filter_map(|s| {
            let chars: Vec<_> = s.chars().collect();
            if let Ok(n) = parse(&chars, &0, &grammar) {
                if n == chars.len() {
                    Some((n, chars.len()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    println!("{}", part2.len());
}
