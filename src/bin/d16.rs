use std::io::{stdin, Read};
use std::ops::RangeInclusive;

use itertools::Itertools;

#[derive(Debug, PartialEq)]
struct Field<'a, T: PartialOrd<T> + PartialEq<T>> {
    name: &'a str,
    range1: RangeInclusive<T>,
    range2: RangeInclusive<T>,
}

impl<T: PartialOrd<T> + PartialEq<T>> Field<'_, T> {
    fn contains(&self, v: &T) -> bool {
        self.range1.contains(v) || self.range2.contains(v)
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let (fields, yours, tickets): (&str, &str, &str) = input.split("\n\n").collect_tuple().unwrap();

    let fields: Vec<_> = fields
        .lines()
        .map(|l| {
            let (name, ranges): (&str, &str) = l.split(": ").collect_tuple().unwrap();
            let (range1, range2) = ranges
                .split(" or ")
                .map(|x| {
                    let (low, high) = x
                        .split('-')
                        .map(|v| v.parse::<u32>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    low..=high
                })
                .collect_tuple()
                .unwrap();
            Field {
                name,
                range1,
                range2,
            }
        })
        .collect();

    let yours: Vec<_> = yours.lines().nth(1).unwrap().split(',').map(|x|x.parse::<u32>().unwrap()).collect();
    let tickets: Vec<Vec<u32>> = tickets
        .lines()
        .skip(1)
        .map(|l| l.split(',').map(|x| x.parse::<u32>().unwrap()).collect())
        .collect();

    let invalid_nums: u32 = tickets
        .iter()
        .flat_map(|r| {
            r.iter()
                .filter(|&&v| !fields.iter().any(|f| f.contains(&v)))
        })
        .sum();
    println!("{:?}", invalid_nums);

    // Remove invalid tickets
    let valid_tickets: Vec<_> = tickets
        .iter()
        .filter(|t| t.iter().all(|v| fields.iter().any(|f| f.contains(v))))
        .collect();

    let mut column_fields = Vec::new();
    for c in 0..fields.len() {
        let valid_fields: Vec<_> = fields
            .iter()
            .filter(|f| valid_tickets.iter().all(|&t| f.contains(&t[c]))).collect();
        column_fields.push(valid_fields);
    }
    let mut final_fields = vec![None; fields.len()];
    for _ in 0..fields.len() {
        for i in 0..column_fields.len() {
            if column_fields[i].len() == 1 {
                let field = column_fields[i][0];
                final_fields[i] = Some(field);
                for j in 0..column_fields.len() {
                    column_fields[j].retain(|&v| v!=field);
                }
            }
        }
    }
    println!("{:?}", final_fields);
    let mut prod = 1;
    for (i, f) in final_fields.iter().enumerate() {
        let f = f.unwrap();
        if f.name.starts_with("departure") {
            prod *= yours[i] as u64;
        }
    }
    println!("{}", prod);

}
