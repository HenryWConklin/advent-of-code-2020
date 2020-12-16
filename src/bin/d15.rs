use std::collections::HashMap;

fn solve1() {
    let mut nums = vec![14, 3, 1, 0, 9, 5];
    while nums.len() < 2020 {
        let last_ind = nums.len() - 1;
        let prev = nums[last_ind];
        let prev_ind = nums[..last_ind].iter().rposition(|&x| x == prev);
        match prev_ind {
            Some(x) => nums.push(last_ind - x),
            None => nums.push(0),
        }
    }
    println!("{:?}", &nums[..20]);
    println!("{}", nums[nums.len() - 1])
}

fn solve2() {
    let mut nums = vec![14, 3, 1, 0, 9, 5];
    let mut prev_seen = HashMap::new();
    for (i, v) in nums.iter().take(nums.len() - 1).enumerate() {
        prev_seen.insert(*v, i);
    }
    while nums.len() < 30000000 {
        let last_ind = nums.len() - 1;
        let prev = nums[last_ind];
        match prev_seen.get(&prev) {
            Some(&i) => {
                nums.push(last_ind - i);
            }
            None => {
                nums.push(0);
            }
        }
        prev_seen.insert(prev, last_ind);
    }
    println!("{:?}", &nums[..20]);
    println!("{}", nums[nums.len() - 1])
}
fn main() {
    solve1();
    solve2();
}
