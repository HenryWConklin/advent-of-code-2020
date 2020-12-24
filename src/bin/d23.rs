use std::collections::VecDeque;

use std::ops::{Add, Rem, Sub};

const POWERS: [u64; 10] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
];

/// Round `n` down to the nearest multiple of `factor`
fn round(n: u64, factor: u64) -> u64 {
    (n / factor) * factor
}

/// Get the `digit`th digit from `n` where 0 is the least significant digit
fn get_digit(n: u64, digit: usize) -> u64 {
    (n / POWERS[digit]) % 10
}

fn mod_sub<T: Copy + Add<Output = T> + Sub<Output = T> + Rem<Output = T>>(a: T, b: T, m: T) -> T {
    ((m + a) - b) % m
}

fn part1() {
    // Part 1
    // Do it all with integer manipulation, just for fun
    let mut num = 398254716;
    const NUM_DIGITS: usize = 9;
    const WINDOW_SIZE: usize = 3;

    for _ in 0..100 {
        let curr_digit = get_digit(num, NUM_DIGITS - 1);

        // Find the target digit
        let mut target = WINDOW_SIZE + 1;
        let mut best_diff = mod_sub(curr_digit, get_digit(num, target), NUM_DIGITS as u64);
        for j in WINDOW_SIZE + 2..9 {
            let d = get_digit(num, NUM_DIGITS - 1 - j);
            let diff = mod_sub(curr_digit, d, NUM_DIGITS as u64);
            if diff < best_diff {
                target = j;
                best_diff = diff;
            }
        }

        let window_digits = num / POWERS[NUM_DIGITS - WINDOW_SIZE - 1] % POWERS[WINDOW_SIZE];

        let before_target = round(
            num % POWERS[NUM_DIGITS - 1 - WINDOW_SIZE],
            POWERS[NUM_DIGITS - 1 - target],
        ) * POWERS[WINDOW_SIZE];
        let window_digits = window_digits * POWERS[NUM_DIGITS - target - 1];
        let after_target = num % POWERS[NUM_DIGITS - 1 - target];
        num = (before_target + window_digits + after_target) * POWERS[1] + curr_digit;
    }
    println!("{}", num);
}

fn part2() {
    // Brute force it is
    const ITERS: usize = 10_000_000;
    const NUM_CUPS: usize = 1_000_000;
    const WINDOW_SIZE: usize = 3;
    let mut nums: VecDeque<u32> = VecDeque::new();
    nums.reserve(NUM_CUPS);
    nums.extend([3, 9, 8, 2, 5, 4, 7, 1, 6].iter());
    nums.extend(10..=NUM_CUPS as u32);
    assert_eq!(nums.len(), NUM_CUPS);

    let mut window = [0; WINDOW_SIZE];
    for iter in 0..ITERS {
        if iter % 1000 == 0 {
            println!("{}", iter);
        }
        let curr_digit = nums.pop_front().unwrap();

        for i in 0..WINDOW_SIZE {
            window[i] = nums.pop_front().unwrap();
        }

        // Only 4 possible target numbers, curr_digit - 1, -2, -3, and -4. Take the largest one
        let mut target = (curr_digit + NUM_CUPS as u32 - 2) % NUM_CUPS as u32 + 1;
        while window.contains(&target) {
            target = (target + NUM_CUPS as u32 - 2) % NUM_CUPS as u32 + 1;
        }

        let target_ind = match nums.iter().position(|&x| x == target) {
            Some(x) => x,
            None => panic!("Invalid target: {}", target),
        };

        // Insert the window values
        for i in 0..WINDOW_SIZE {
            nums.insert(target_ind + i + 1, window[i]);
        }

        nums.push_back(curr_digit);
    }
    println!();
    let ind = nums.iter().position(|&x| x == 1).unwrap();
    println!(
        "{} {} {}",
        nums[ind + 1],
        nums[ind + 2],
        nums[ind + 1] as u64 * nums[ind + 2] as u64
    );
}

fn main() {
    part1();
    part2();
}
