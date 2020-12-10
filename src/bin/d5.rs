use std::io;
use std::io::BufRead;

use itertools::Itertools;

fn main() {
    let mut inds: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'B' => '1',
                    'F' => '0',
                    'L' => '0',
                    'R' => '1',
                    _ => panic!("invalid char"),
                })
                .collect::<String>()
        })
        .map(|l| {
            (
                u32::from_str_radix(&l[..7], 2).unwrap(),
                u32::from_str_radix(&l[7..], 2).unwrap(),
            )
        })
        .map(|(a, b)| (a * 8) + b)
        .collect();

    let max_ind = inds.iter().max().unwrap();
    println!("{}", max_ind);

    inds.sort_unstable();
    let (a, _) = inds
        .iter()
        .tuple_windows()
        .filter(|(a, b)| *b - *a == 2)
        .next()
        .unwrap();
    println!("{}", a + 1);
}
