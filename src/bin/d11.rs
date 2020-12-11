use std::io::{stdin, BufRead};
use std::mem::swap;

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Floor,
    EmptySeat,
    Occupied,
}

type Grid = [Vec<Cell>];

fn adj_count(g: &Grid, i: usize, j: usize) -> usize {
    let mut adj = 0;

    let row_range = if i == 0 {
        i..=i + 1
    } else if i == g.len() - 1 {
        i - 1..=i
    } else {
        i - 1..=i + 1
    };
    let col_range = if j == 0 {
        j..=j + 1
    } else if j == g[i].len() - 1 {
        j - 1..=j
    } else {
        j - 1..=j + 1
    };
    for a in row_range {
        for b in col_range.clone() {
            if a == i && b == j {
                continue;
            }
            if g[a][b] == Cell::Occupied {
                adj += 1;
            }
        }
    }
    adj
}

fn beam_adj_count(g: &Grid, i: usize, j: usize) -> usize {
    const SLOPES: [(isize, isize); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let mut adj_count = 0;
    for (a, b) in &SLOPES {
        let mut ci = (i as isize) + a;
        let mut cj = (j as isize) + b;
        while ci >= 0 && cj >= 0 && (ci as usize) < g.len() && (cj as usize) < g[ci as usize].len()
        {
            match g[ci as usize][cj as usize] {
                Cell::Occupied => {
                    adj_count += 1;
                    break;
                }
                Cell::EmptySeat => {
                    break;
                }
                Cell::Floor => {}
            }
            ci += a;
            cj += b;
        }
    }
    adj_count
}

fn step_grid<F: Fn(&Grid, usize, usize) -> usize>(
    g: &Grid,
    ng: &mut Grid,
    adj_count: &F,
    kill_thresh: usize,
) -> bool {
    let mut changed = false;
    for (i, row) in g.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            match c {
                Cell::Floor => {}
                Cell::Occupied => {
                    if adj_count(g, i, j) >= kill_thresh {
                        ng[i][j] = Cell::EmptySeat;
                        changed = true;
                    } else {
                        ng[i][j] = Cell::Occupied;
                    }
                }
                Cell::EmptySeat => {
                    if adj_count(g, i, j) == 0 {
                        ng[i][j] = Cell::Occupied;
                        changed = true;
                    } else {
                        ng[i][j] = Cell::EmptySeat;
                    }
                }
            }
        }
    }
    changed
}

fn occupied_count(g: &Grid) -> usize {
    g.iter()
        .map(|r| r.iter().filter(|c| **c == Cell::Occupied).count())
        .sum()
}

fn main() {
    let mut grid: Vec<Vec<Cell>> = stdin()
        .lock()
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| match c {
                    'L' => Cell::EmptySeat,
                    '.' => Cell::Floor,
                    _ => Cell::Floor,
                })
                .collect()
        })
        .collect();

    let mut grid2 = grid.clone();
    let mut back_grid = grid.clone();

    while step_grid(&grid, &mut back_grid, &adj_count, 4) {
        swap(&mut grid, &mut back_grid)
    }
    println!("{}", occupied_count(&grid));

    while step_grid(&grid2, &mut back_grid, &beam_adj_count, 5) {
        swap(&mut grid2, &mut back_grid);
    }
    println!("{}", occupied_count(&grid2));
}
