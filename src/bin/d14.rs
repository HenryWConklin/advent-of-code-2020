use core::fmt;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::{stdin, BufRead};

use regex::Regex;

type MemVal = u64;

const BITS: MemVal = 36;

#[derive(Copy, Clone)]
enum Instruction {
    Mask { mask: MemVal, value: MemVal },
    Set { addr: MemVal, value: MemVal },
}

#[derive(Debug)]
struct MemState {
    mask: MemVal,
    mask_value: MemVal,
    memory: HashMap<MemVal, MemVal>,
}

impl MemState {
    fn instruction(mut self, instruction: &Instruction) -> Self {
        match instruction {
            Instruction::Mask { mask, value } => {
                self.mask = *mask;
                self.mask_value = *value;
            }
            Instruction::Set { addr, value } => {
                self.memory.insert(
                    *addr,
                    (value & (self.mask)) | (!self.mask & self.mask_value),
                );
            }
        }
        self
    }
}

#[derive(Debug)]
struct FloatMem {
    float_mask: MemVal,
    addr: MemVal,
    value: MemVal,
}

impl Display for FloatMem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{float: {:036b}, addr: {:036b}, value: {}}}",
            self.float_mask, self.addr, self.value
        )
    }
}

struct MemState2 {
    float_mask: MemVal,
    float_addrs: Vec<MemVal>,
    set_bits: MemVal,
    memory: HashMap<MemVal, MemVal>,
}

impl MemState2 {
    fn instruction(mut self, instruction: &Instruction) -> Self {
        match instruction {
            Instruction::Mask { mask, value } => {
                self.float_mask = *mask;
                self.float_addrs = addrs(mask);
                self.set_bits = *value;
            }
            Instruction::Set { addr, value } => {
                for a in self.float_addrs.iter() {
                    self.memory.insert(
                        (addr & !self.float_mask) | (a & self.float_mask) | self.set_bits,
                        *value,
                    );
                }
            }
        }
        self
    }
}

fn addrs(mask: &MemVal) -> Vec<MemVal> {
    let inds: Vec<MemVal> = (0..BITS).filter(|b| mask & (1 << b) != 0).collect();
    let mut res = Vec::new();

    for v in 0..(1 << inds.len()) {
        let mut val = 0;
        for (ind, shift) in inds.iter().enumerate() {
            val |= (v & (1 << ind)) << (*shift - ind as MemVal);
        }
        res.push(val);
    }

    res
}

fn read_instructions<T: Iterator<Item=String>>(lines: T) -> Vec<Instruction>{
    let set_regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    let instructions: Vec<_> = lines
        .map(|l| {
            if l.starts_with("mask") {
                let mask_str = &l[7..];
                let mask = MemVal::from_str_radix(
                    mask_str
                        .chars()
                        .map(|c| match c {
                            'X' => '1',
                            _ => '0',
                        })
                        .collect::<String>()
                        .as_str(),
                    2,
                )
                    .unwrap();
                let value = MemVal::from_str_radix(
                    mask_str
                        .chars()
                        .map(|c| match c {
                            '1' => '1',
                            _ => '0',
                        })
                        .collect::<String>()
                        .as_str(),
                    2,
                )
                    .unwrap();
                Instruction::Mask { mask, value }
            } else if l.starts_with("mem") {
                let captures = set_regex.captures(&l).unwrap();
                Instruction::Set {
                    addr: captures.get(1).unwrap().as_str().parse().unwrap(),
                    value: captures.get(2).unwrap().as_str().parse().unwrap(),
                }
            } else {
                panic!("Invalid instruction")
            }
        })
        .collect();
    instructions
}

fn solve1(instructions: &[Instruction]) {
    let result = instructions.iter().fold(
        MemState {
            mask: 0,
            mask_value: 0,
            memory: HashMap::new(),
        },
        |acc, v| acc.instruction(&v),
    );
    println!("{}", result.memory.values().sum::<MemVal>());
}

fn solve2(instructions: &[Instruction]) {
    let result2 = instructions.iter().fold(
        MemState2 {
            float_mask: 0,
            float_addrs: Vec::new(),
            set_bits: 0,
            memory: HashMap::new(),
        },
        |acc, v| acc.instruction(v),
    );
    println!("{}", result2.memory.values().sum::<MemVal>());
}

fn main() {
    let instructions = read_instructions(stdin().lock().lines().map(|x|x.unwrap()));
    solve1(&instructions);
    solve2(&instructions);
}

#[test]
fn test_mask_addrs() {
    assert_eq!(vec![0,1,2,3], addrs(&0b11));
    assert_eq!(vec![0,1,2,3,4,5,6,7], addrs(&0b111));
    assert_eq!(vec![0b000, 0b001, 0b100, 0b101], addrs(&0b101));
}
