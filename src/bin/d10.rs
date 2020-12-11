use std::collections::HashMap;
use std::io::{stdin, BufRead};

use itertools::Itertools;
use std::ops::Add;

fn calc_ncombos<T: Add<Output = T> + Copy>(ncombos: &mut Vec<T>, n: usize) -> T {
    while ncombos.len() <= n {
        let l = ncombos.len();
        ncombos.push(ncombos[l - 1] + ncombos[l - 2] + ncombos[l - 3]);
    }
    ncombos[n]
}

fn main() {
    let nums = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect_vec();

    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    sorted_nums.push(sorted_nums[sorted_nums.len() - 1] + 3);

    let mut diff_counts = HashMap::new();
    let mut run_length = 0;
    let mut total_combos: i64 = 1;
    let mut ncombos = vec![1, 1, 2];
    for v in sorted_nums.iter().scan(0, |prev, x| {
        let res = Some(x - *prev);
        *prev = *x;
        res
    }) {
        // Count occurences of each difference value
        match diff_counts.get(&v).cloned() {
            Some(c) => diff_counts.insert(v, c + 1),
            None => diff_counts.insert(v, 1),
        };

        // Count up the run lengths of runs of 1s
        if v == 1 {
            run_length += 1;
        } else {
            // Each run of 1s can be substituted f(n) = f(n-1) + f(n-2) + f(n-3) different ways
            // Total combinations is the product of all of these values
            // Last diff is guaranteed to be a 3, so don't need to handle the last run separately
            total_combos *= calc_ncombos(&mut ncombos, run_length);
            run_length = 0;
        }
    }
    println!(
        "{:?} {}",
        diff_counts,
        diff_counts.get(&1).unwrap() * diff_counts.get(&3).unwrap()
    );
    println!("{}", total_combos);
}
