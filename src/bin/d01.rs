use std::collections::HashSet;
use std::io;

use itertools::Itertools;

fn main() {
    let mut set = HashSet::new();
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => (),
            Err(error) => panic!(error),
        }

        let val: i32 = match line.trim() {
            "" => continue,
            x => x.parse().unwrap(),
        };
        set.insert(val);
    }

    println!("Part 1");
    for val in &set {
        let other = 2020 - val;
        if set.contains(&other) {
            println!("{} {} {}", val, other, val * other);
            break;
        }
    }

    println!("Part 2");
    for pair in set.iter().cloned().combinations(2) {
        let a = pair.first().unwrap();
        let b = pair.last().unwrap();
        let other = 2020 - a - b;
        if set.contains(&other) {
            println!("{} {} {} {}", a, b, other, a * b * other);
            break;
        }
    }
}
