use std::collections::HashSet;
use std::io::{stdin, BufRead};

fn part1(lines: Vec<String>) -> HashSet<(i32, i32)> {
    let mut flipped = HashSet::new();
    for l in lines {
        let mut chars = l.chars();
        let mut row: i32 = 0;
        let mut col: i32 = 0;
        while let Some(c) = chars.next() {
            if c == 'n' || c == 's' {
                let c2 = chars
                    .next()
                    .unwrap_or_else(|| panic!("Invalid direction '{}'", c));
                match c2 {
                    'e' => {
                        if row.rem_euclid(2) == 0 {
                            col += 1
                        }
                    }
                    'w' => {
                        if row.rem_euclid(2) == 1 {
                            col -= 1
                        }
                    }
                    _ => panic!("Invalid direction '{}{}'", c, c2),
                }
                if c == 'n' {
                    row -= 1;
                } else {
                    row += 1;
                }
            } else if c == 'e' {
                col += 1;
            } else if c == 'w' {
                col -= 1;
            } else {
                panic!("Invalid direction {}", c);
            }
        }

        // Remove if present, insert if not present
        if !flipped.remove(&(row, col)) {
            flipped.insert((row, col));
        }
    }
    println!("{}", flipped.len());
    flipped
}

fn adj_count(row: i32, col: i32, tiles: &HashSet<(i32, i32)>) -> u8 {
    adjacent(row, col)
        .iter()
        .filter(|&x| tiles.contains(x))
        .count() as u8
}

fn adjacent(row: i32, col: i32) -> [(i32, i32); 6] {
    let col_offset = row.rem_euclid(2);
    [
        (row - 1, col - col_offset),
        (row - 1, col + 1 - col_offset),
        (row, col - 1),
        (row, col + 1),
        (row + 1, col - col_offset),
        (row + 1, col + 1 - col_offset),
    ]
}

fn part2(init: HashSet<(i32, i32)>) {
    let mut tiles = init;
    for _ in 0..100 {
        let mut next_tiles = HashSet::new();
        for t in tiles.iter() {
            for (r, c) in &adjacent(t.0, t.1) {
                if !tiles.contains(&(*r, *c))
                    && !next_tiles.contains(&(*r, *c))
                    && adj_count(*r, *c, &tiles) == 2
                {
                    next_tiles.insert((*r, *c));
                }
            }

            let curr_adj_count = adj_count(t.0, t.1, &tiles);
            if curr_adj_count == 1 || curr_adj_count == 2 {
                next_tiles.insert(*t);
            }
        }
        tiles = next_tiles;
    }
    println!("{}", tiles.len());
}

fn main() {
    let lines: Vec<_> = stdin().lock().lines().map(|l| l.unwrap()).collect();
    let tiles = part1(lines);
    part2(tiles);
}
