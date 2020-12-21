use std::collections::HashMap;
use std::io::{stdin, Read};
use std::mem::swap;

const TOP: usize = 0;
const LEFT: usize = 1;
const BOTTOM: usize = 2;
const RIGHT: usize = 3;
const SEA_MONSTER: [&str; 3] = [
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   ",
];

struct Frame {
    id: u64,
    grid: [[bool; 8]; 8],
    top: u16,
    left: u16,
    bottom: u16,
    right: u16,
}

impl Frame {
    fn border_to_int<T: Iterator<Item = char>>(border: T) -> u16 {
        let mut acc = 0;
        for c in border {
            acc <<= 1;
            match c {
                '#' => acc += 1,
                '.' => {}
                _ => panic!("Invalid character"),
            }
        }
        acc
    }

    fn new(id: u64, grid: &str) -> Frame {
        let top = Self::border_to_int(grid.lines().next().unwrap().chars());
        let bottom = Self::border_to_int(grid.lines().last().unwrap().chars());
        let left = Self::border_to_int(grid.lines().map(|l| l.chars().next().unwrap()));
        let right = Self::border_to_int(grid.lines().map(|l| l.chars().last().unwrap()));

        let mut n_grid = [[false; 8]; 8];

        let grid = grid.lines().skip(1).take(8).map(|l| {
            l.chars().skip(1).take(8).map(|c| match c {
                '#' => true,
                '.' => false,
                _ => panic!("Invalid char"),
            })
        });
        for (i, row) in grid.enumerate() {
            for (j, v) in row.enumerate() {
                n_grid[i][j] = v;
            }
        }

        Frame {
            id,
            grid: n_grid,
            top,
            bottom,
            left,
            right,
        }
    }
    fn flip_col(&mut self) {
        for row in &mut self.grid {
            row.reverse();
        }
        swap(&mut self.left, &mut self.right);
        self.top = flip(self.top);
        self.bottom = flip(self.bottom);
    }
    fn flip_row(&mut self) {
        self.grid.reverse();
        swap(&mut self.bottom, &mut self.top);
        self.left = flip(self.left);
        self.right = flip(self.right);
    }

    fn transpose(&mut self) {
        for i in 0..self.grid.len() {
            for j in i + 1..self.grid.len() {
                let temp = self.grid[i][j];
                self.grid[i][j] = self.grid[j][i];
                self.grid[j][i] = temp;
            }
        }
        swap(&mut self.top, &mut self.left);
        swap(&mut self.bottom, &mut self.right);
    }
    fn get_key(&self, dir: usize) -> u16 {
        match dir {
            TOP => self.top,
            LEFT => self.left,
            BOTTOM => self.bottom,
            RIGHT => self.right,
            _ => panic!("Invalid direction index {}", dir),
        }
    }
}

fn flip(x: u16) -> u16 {
    x.reverse_bits() >> 6
}

fn tile_flip_row(i: usize, tiles: &mut [Frame], adj_map: &mut [[Option<(usize, usize)>; 4]]) {
    // Flip the grid + ids
    tiles[i].flip_row();

    // Fix the adj map
    if let Some((j, dj)) = adj_map[i][TOP] {
        adj_map[j][dj] = Some((i, BOTTOM))
    }
    if let Some((j, dj)) = adj_map[i][BOTTOM] {
        adj_map[j][dj] = Some((i, TOP))
    }
    adj_map[i].swap(TOP, BOTTOM);
}

fn tile_flip_col(i: usize, tiles: &mut [Frame], adj_map: &mut [[Option<(usize, usize)>; 4]]) {
    tiles[i].flip_col();

    // Fix the adj map
    if let Some((j, dj)) = adj_map[i][LEFT] {
        adj_map[j][dj] = Some((i, RIGHT))
    }
    if let Some((j, dj)) = adj_map[i][RIGHT] {
        adj_map[j][dj] = Some((i, LEFT))
    }
    adj_map[i].swap(LEFT, RIGHT);
}

fn tile_transpose(i: usize, tiles: &mut [Frame], adj_map: &mut [[Option<(usize, usize)>; 4]]) {
    tiles[i].transpose();

    // Fix the adj map
    if let Some((j, dj)) = adj_map[i][TOP] {
        adj_map[j][dj] = Some((i, LEFT))
    }
    if let Some((j, dj)) = adj_map[i][LEFT] {
        adj_map[j][dj] = Some((i, TOP))
    }
    if let Some((j, dj)) = adj_map[i][BOTTOM] {
        adj_map[j][dj] = Some((i, RIGHT))
    }
    if let Some((j, dj)) = adj_map[i][RIGHT] {
        adj_map[j][dj] = Some((i, BOTTOM))
    }
    adj_map[i].swap(TOP, LEFT);
    adj_map[i].swap(BOTTOM, RIGHT);
}

/// Fix the transform of the tile adjacent to the given tile in the given direction
fn fix_transform(
    ind: usize,
    dir: usize,
    tiles: &mut [Frame],
    adj_map: &mut [[Option<(usize, usize)>; 4]],
) {
    let other = adj_map[ind][dir].unwrap();
    let other_ind = other.0;
    let mut other_dir = other.1;
    // One edge is a vertical edge, the other is horizontal. Transpose to match
    if dir % 2 != other_dir % 2 {
        tile_transpose(other_ind, tiles, adj_map);
        other_dir = adj_map[ind][dir].unwrap().1;
    }

    // Edges should match up 0->2 and 1->3. If they're equal, flip to correct orientation
    if dir == other_dir {
        // Horizontal edge
        if dir % 2 == 0 {
            tile_flip_row(other_ind, tiles, adj_map);
        }
        // Vertical edge
        else {
            tile_flip_col(other_ind, tiles, adj_map);
        }

        other_dir = adj_map[ind][dir].unwrap().1;
    }

    // Edge may be mirrored, flip on other axis to fix
    if tiles[ind].get_key(dir) != tiles[other_ind].get_key(other_dir) {
        // Horizontal edge
        if dir % 2 == 0 {
            tile_flip_col(other_ind, tiles, adj_map);
        }
        // Vertical edge
        else {
            tile_flip_row(other_ind, tiles, adj_map);
        }
    }
}

fn find_sea_monsters(grid: &mut [[bool; 12 * 8]; 12 * 8]) -> usize {
    let sea_monster_cols = SEA_MONSTER[0].len();
    let mut count = 0;

    for i in 0..grid.len() - SEA_MONSTER.len() {
        for j in 0..grid[0].len() - sea_monster_cols {
            let mut found = true;
            for di in 0..SEA_MONSTER.len() {
                for (dj, monster_char) in SEA_MONSTER[di].chars().enumerate() {
                    found = found && (monster_char != '#' || grid[i + di][j + dj]);
                }
            }
            // If found, erase all parts of the sea monster
            if found {
                count += 1;
                for di in 0..SEA_MONSTER.len() {
                    for (dj, monster_char) in SEA_MONSTER[di].chars().enumerate() {
                        if monster_char == '#' {
                            grid[i + di][j + dj] = false;
                        }
                    }
                }
            }
        }
    }
    count
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut tiles: Vec<_> = input
        .trim()
        .split("\n\n")
        .map(|t| {
            let id = t[5..9].parse().unwrap();
            let grid = &t[11..];
            Frame::new(id, grid)
        })
        .collect();

    // Which tiles are adjacent to which other tiles, top left bottom right
    let mut adj_map: Vec<[Option<(usize, usize)>; 4]> = vec![[None; 4]; tiles.len()];
    // Map of edge key => index of frame
    let mut edge_map = HashMap::new();

    let mut insert_adj = |i: usize, di: usize, j: usize, dj: usize| {
        match adj_map[i][di] {
            Some((k, dk)) => {
                if !(k == j && dk == dj) {
                    panic!("Duplicate match: {} {} {} {} {} {}", i, di, j, dj, k, dk)
                }
            }
            None => adj_map[i][di] = Some((j, dj)),
        }
        match adj_map[j][dj] {
            Some((k, dk)) => {
                if !(k == i && dk == di) {
                    panic!("Duplicate match: {} {} {} {} {} {}", i, di, j, dj, k, dk)
                }
            }
            None => adj_map[j][dj] = Some((i, di)),
        }
    };

    for (i, tile) in tiles.iter().enumerate() {
        let dirs = [
            (tile.top, TOP),
            (tile.left, LEFT),
            (tile.bottom, BOTTOM),
            (tile.right, RIGHT),
        ];
        for (border_key, dir) in &dirs {
            if let Some((j, dj)) = edge_map.insert(*border_key, (i, *dir)) {
                insert_adj(i, *dir, j, dj)
            }
            if let Some((j, dj)) = edge_map.insert(flip(*border_key), (i, *dir)) {
                insert_adj(i, *dir, j, dj)
            }
        }
    }

    let corners: Vec<_> = adj_map
        .iter()
        .enumerate()
        // Filter out tiles with two adjacent other tiles (i.e. the corners)
        .filter_map(|(i, adj)| {
            let adj_count = adj.iter().filter(|&&x| x != None).count();
            if adj_count == 2 {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    // Find the product of their IDs
    let part1: u64 = corners.iter().map(|t| tiles[*t].id).product();
    println!("{}", part1);

    // 144 sub-grids in input, square image so 12x12, each sub-grid has edges removed so now 8x8
    let mut full_grid = [[false; 12 * 8]; 12 * 8];
    let mut next_row = corners[0];

    // Fix the transform on the first corner, want adjacent tiles to right and bottom
    if adj_map[next_row][TOP] != None {
        tile_flip_row(next_row, &mut tiles, &mut adj_map);
    }
    if adj_map[next_row][LEFT] != None {
        tile_flip_col(next_row, &mut tiles, &mut adj_map);
    }

    for i in 0..12 {
        let mut curr = next_row;
        if let Some((ind, _)) = adj_map[next_row][BOTTOM] {
            fix_transform(curr, BOTTOM, &mut tiles, &mut adj_map);
            next_row = ind;
        }
        for j in 0..12 {
            // Copy to final grid
            for grid_row in 0..8 {
                full_grid[i * 8 + grid_row][j * 8..(j + 1) * 8]
                    .copy_from_slice(&tiles[curr].grid[grid_row]);
            }

            // Fix transform of next tile
            if let Some((ind, _)) = adj_map[curr][RIGHT] {
                fix_transform(curr, RIGHT, &mut tiles, &mut adj_map);
                curr = ind;
            }
        }
    }

    // Try all 8 transformations of the image
    'outer: for _ in 0..2 {
        for _ in 0..2 {
            for _ in 0..2 {
                let sea_monster_count = find_sea_monsters(&mut full_grid);
                if sea_monster_count > 0 {
                    println!("Found {} sea monsters", sea_monster_count);
                    break 'outer;
                }

                // Flip rows
                full_grid.reverse();
            }
            // Flip cols
            for i in 0..full_grid.len() {
                full_grid[i].reverse();
            }
        }
        // Transpose
        for i in 0..full_grid.len() {
            for j in i + 1..full_grid.len() {
                let temp = full_grid[i][j];
                full_grid[i][j] = full_grid[j][i];
                full_grid[j][i] = temp;
            }
        }
    }
    let roughness = full_grid
        .iter()
        .flat_map(|r| r.iter())
        .filter(|&&x| x)
        .count();
    println!("{}", roughness);
}

#[test]
fn nest_arr_reverse() {
    let mut arr = [[1, 2, 3], [4, 5, 6]];
    arr.reverse();
    assert_eq!(arr, [[4, 5, 6], [1, 2, 3]])
}
