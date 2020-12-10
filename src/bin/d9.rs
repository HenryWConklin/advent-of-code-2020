use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead};

use itertools::Itertools;

fn can_sum_two(vals: &HashSet<i64>, target: &i64) -> bool {
    for v in vals {
        if vals.contains(&(target - v)) {
            return true;
        }
    }
    false
}

fn main() {
    let nums = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect_vec();
    let mut window = HashSet::new();
    let window_size = 25;
    window.extend(nums[..window_size].iter());
    let mut invalid_num = None;
    for (i, x) in nums[window_size..].iter().enumerate() {
        if !can_sum_two(&window, x) {
            invalid_num = Some(x);
        }
        // i is window_size values smaller than in should be, but want the value window_size
        // indices ago so it all cancels out
        window.remove(&nums[i]);
        window.insert(*x);
    }
    let invalid_num = invalid_num.unwrap();
    println!("{}", invalid_num);

    // Cumulative sum along the array
    let cumsum = nums
        .iter()
        .scan(0, |acc, &x| {
            *acc = *acc + x;
            Some(*acc)
        })
        .collect_vec();

    // Map values -> indices
    let mut cumsum_rev = HashMap::new();
    cumsum_rev.extend(cumsum.iter().enumerate().map(|(a, b)| (b, a)));
    let result = match cumsum_rev.get(invalid_num) {
        Some(ind) => (0, ind + 1),
        None => {
            let mut result = None;
            for (i, v) in cumsum.iter().enumerate() {
                match cumsum_rev.get(&(v + invalid_num)) {
                    Some(ind) => {
                        if ind - i >= 2 {
                            result = Some((i + 1, ind + 1));
                            break;
                        }
                    }
                    None => {}
                }
            }
            result.unwrap()
        }
    };
    let subarr = &nums[result.0..result.1];
    println!("{:?} {:?} {}", result, subarr, subarr.iter().sum::<i64>());
    println!(
        "{}",
        subarr.iter().max().unwrap() + subarr.iter().min().unwrap()
    )
}
