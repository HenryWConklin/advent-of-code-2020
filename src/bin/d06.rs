use std::collections::HashSet;
use std::io::{stdin, BufRead};

fn main() {
    let mut answer_union = HashSet::new();
    let mut answer_intersection = HashSet::new();
    let mut answer_union_total = 0;
    let mut answer_intersection_total = 0;
    let mut group_start = true;
    for line in stdin().lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            answer_union_total += answer_union.len();
            answer_union.clear();
            answer_intersection_total += answer_intersection.len();
            answer_intersection.clear();
            group_start = true;
        } else {
            answer_union.extend(line.chars());
            if group_start {
                answer_intersection.extend(line.chars());
            } else {
                answer_intersection = answer_intersection
                    .intersection(&line.chars().collect())
                    .cloned()
                    .collect();
            }
            group_start = false;
        }
    }
    answer_union_total += answer_union.len();
    answer_intersection_total += answer_intersection.len();
    println!("{}", answer_union_total);
    println!("{}", answer_intersection_total);
}
