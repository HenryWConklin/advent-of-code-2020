use std::collections::HashMap;
use std::io;
use std::io::BufRead;

use itertools::Itertools;

fn collect_lines<I>(i: &mut I) -> Option<HashMap<String, String>>
where
    I: Iterator<Item = String>,
{
    let first = match i.next() {
        Some(x) => x,
        None => return None,
    };
    let mut res = HashMap::new();
    fn insert_pairs(x: String, res: &mut HashMap<String, String>) {
        for s in x.split_whitespace() {
            let ind = s.find(':').expect("Invalid token");
            res.insert(s[..ind].to_owned(), s[ind + 1..].to_owned());
        }
    }
    insert_pairs(first, &mut res);
    for next in i {
        if next.is_empty() {
            break;
        }
        insert_pairs(next, &mut res);
    }
    Some(res)
}

fn valid_range_num(low: usize, high: usize, v: &str) -> bool {
    match v.parse() {
        Ok(x) => (low..=high).contains(&x),
        Err(_) => false,
    }
}

fn main() {
    let stdin = io::stdin();
    let passports: Vec<_> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .batching(collect_lines)
        .collect();
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let num_valid1 = passports
        .iter()
        .filter(|h| required_fields.iter().all(|k| h.contains_key(*k)))
        .count();
    println!("{}", num_valid1);
    let valid_eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let num_valid2 = passports
        .iter()
        .filter(|h| required_fields.iter().all(|k| h.contains_key(*k)))
        .filter(|h| valid_range_num(1920, 2002, h.get("byr").unwrap()))
        .filter(|h| valid_range_num(2010, 2020, h.get("iyr").unwrap()))
        .filter(|h| valid_range_num(2020, 2030, h.get("eyr").unwrap()))
        .filter(|h| {
            let val = h.get("hgt").unwrap();
            let num: String = val.chars().take(val.len() - 2).collect();
            if val.ends_with("cm") {
                valid_range_num(150, 193, &num)
            } else if val.ends_with("in") {
                valid_range_num(59, 76, &num)
            } else {
                false
            }
        })
        .filter(|h| {
            let val = h.get("hcl").unwrap();
            val.len() == 7
                && val.chars().nth(0).unwrap() == '#'
                && val
                    .chars()
                    .skip(1)
                    .all(|c| ('a'..='f').contains(&c) || c.is_ascii_digit())
        })
        .filter(|h| valid_eye_colors.contains(&h.get("ecl").unwrap().as_str()))
        .filter(|h| {
            let val = h.get("pid").unwrap();
            val.len() == 9 && val.chars().all(|c| c.is_ascii_digit())
        })
        .count();

    println!("{}", num_valid2)
}
