use std::io::{stdin, BufRead};

fn slope_count(g: &[Vec<bool>], r: usize, c: usize) -> usize {
    g.iter()
        .step_by(r)
        .enumerate()
        .map(|(rind, row)| row[(rind * c) % g[0].len()])
        .filter(|x| *x)
        .count()
}

fn main() {
    let grid: Vec<Vec<bool>> = stdin()
        .lock()
        .lines()
        .map(|r| {
            r.unwrap()
                .chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => false,
                })
                .collect()
        })
        .collect();
    println!("{}", slope_count(&grid, 1, 3));

    let part2: usize = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(c, r)| slope_count(&grid, *r, *c))
        .product();
    println!("{}", part2);
}
