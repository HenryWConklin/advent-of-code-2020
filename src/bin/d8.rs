use std::collections::HashSet;
use std::io::{stdin, BufRead};

fn step_prog(instructions: &Vec<(String, i32)>, pc: &mut usize, acc: &mut i32) {
    let mut next = *pc + 1;
    let (op, v) = &instructions[*pc];
    match op.as_str() {
        "nop" => {}
        "acc" => *acc += v,
        "jmp" => next = (*pc as i32 + v) as usize,
        _ => panic!("Invalid instruction"),
    }
    *pc = next;
}

fn run_prog(instructions: &Vec<(String, i32)>, start: usize, acc: i32) -> Result<i32, i32> {
    let mut pc = start;
    let mut acc = acc;
    let mut visited = HashSet::new();
    while pc < instructions.len() && !visited.contains(&pc) {
        visited.insert(pc);
        step_prog(&instructions, &mut pc, &mut acc);
    }
    match visited.contains(&pc) {
        true => Err(acc),
        false => Ok(acc),
    }
}

fn main() {
    let instructions: Vec<_> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| (l[..3].to_owned(), l[4..].parse::<i32>().unwrap()))
        .collect();

    println!("{:?}", run_prog(&instructions, 0, 0));

    let mut pc = 0;
    let mut acc = 0;
    loop {
        let (op, v) = &instructions[pc];
        match op.as_str() {
            "nop" => match run_prog(&instructions, (pc as i32 + v) as usize, acc) {
                Ok(v) => {
                    acc = v;
                    break;
                }
                _ => {}
            },
            "jmp" => match run_prog(&instructions, pc + 1, acc) {
                Ok(v) => {
                    acc = v;
                    break;
                }
                _ => {}
            },
            _ => {}
        }
        step_prog(&instructions, &mut pc, &mut acc);
    }
    println!("{}", acc);
}
