use core::fmt;
use std::fmt::{Display, Formatter};
use std::io::{stdin, BufRead};

#[derive(Copy, Clone, PartialEq, Debug)]
enum CellState {
    Active,
    Inactive,
}

#[derive(Debug)]
struct Grid {
    data: Vec<CellState>,
    minx: i32,
    miny: i32,
    minz: i32,
    maxx: i32,
    maxy: i32,
    maxz: i32,
}

impl Grid {
    fn new(init: &[Vec<CellState>]) -> Grid {
        let mut data = Vec::new();
        data.reserve(init.len() * init[0].len());

        for row in init {
            for v in row {
                data.push(*v);
            }
        }

        Grid {
            data,
            minx: 0,
            miny: 0,
            minz: 0,
            maxx: init.len() as i32,
            maxy: init[0].len() as i32,
            maxz: 1,
        }
    }

    fn empty_expand(g: &Grid) -> Grid {
        let size = (g.maxx - g.minx + 2) * (g.maxy - g.miny + 2) * (g.maxz - g.minz + 2);
        let data = vec![CellState::Inactive; size as usize];
        Grid {
            data,
            minx: g.minx - 1,
            maxx: g.maxx + 1,
            miny: g.miny - 1,
            maxy: g.maxy + 1,
            minz: g.minz - 1,
            maxz: g.maxz + 1,
        }
    }

    fn get_ind(&self, i: i32, j: i32, k: i32) -> Option<usize> {
        if i < self.minx
            || i >= self.maxx
            || j < self.miny
            || j >= self.maxy
            || k < self.minz
            || k >= self.maxz
        {
            return None;
        }
        let ywidth = self.maxy - self.miny;
        let zwidth = self.maxz - self.minz;
        let ind = (i - self.minx) * ywidth * zwidth + (j - self.miny) * zwidth + (k - self.minz);
        Some(ind as usize)
    }

    fn get(&self, i: i32, j: i32, k: i32) -> CellState {
        match self.get_ind(i, j, k) {
            None => CellState::Inactive,
            Some(ind) => self.data[ind],
        }
    }

    fn set(&mut self, i: i32, j: i32, k: i32, v: CellState) {
        match self.get_ind(i, j, k) {
            None => panic!("Set on out of bounds index"),
            Some(ind) => self.data[ind] = v,
        }
    }

    fn active_neighbors(&self, i: i32, j: i32, k: i32) -> u8 {
        let mut count = 0;

        for di in -1..=1 {
            for dj in -1..=1 {
                for dk in -1..=1 {
                    if di == 0 && dj == 0 && dk == 0 {
                        continue;
                    }

                    if self.get(i + di, j + dj, k + dk) == CellState::Active {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    fn step(&self) -> Grid {
        let mut new_grid = Grid::empty_expand(self);
        for i in new_grid.minx..new_grid.maxx {
            for j in new_grid.miny..new_grid.maxy {
                for k in new_grid.minz..new_grid.maxz {
                    let count = self.active_neighbors(i, j, k);
                    match self.get(i, j, k) {
                        CellState::Active => {
                            if count == 2 || count == 3 {
                                new_grid.set(i, j, k, CellState::Active)
                            }
                        }
                        CellState::Inactive => {
                            if count == 3 {
                                new_grid.set(i, j, k, CellState::Active)
                            }
                        }
                    }
                }
            }
        }
        new_grid
    }
}

#[derive(Debug)]
struct Grid4 {
    data: Vec<CellState>,
    minx: i32,
    miny: i32,
    minz: i32,
    maxx: i32,
    maxy: i32,
    maxz: i32,
    minw: i32,
    maxw: i32,
}

impl Grid4 {
    fn new(init: &[Vec<CellState>]) -> Grid4 {
        let mut data = Vec::new();
        data.reserve(init.len() * init[0].len());

        for row in init {
            for v in row {
                data.push(*v);
            }
        }

        Grid4 {
            data,
            minx: 0,
            miny: 0,
            minz: 0,
            maxx: init.len() as i32,
            maxy: init[0].len() as i32,
            maxz: 1,
            minw: 0,
            maxw: 1,
        }
    }

    fn empty_expand(g: &Grid4) -> Grid4 {
        let size = (g.maxx - g.minx + 2)
            * (g.maxy - g.miny + 2)
            * (g.maxz - g.minz + 2)
            * (g.maxw - g.minw + 2);
        let data = vec![CellState::Inactive; size as usize];
        Grid4 {
            data,
            minx: g.minx - 1,
            maxx: g.maxx + 1,
            miny: g.miny - 1,
            maxy: g.maxy + 1,
            minz: g.minz - 1,
            maxz: g.maxz + 1,
            minw: g.minw - 1,
            maxw: g.maxw + 1,
        }
    }

    fn get_ind(&self, i: i32, j: i32, k: i32, w: i32) -> Option<usize> {
        if i < self.minx
            || i >= self.maxx
            || j < self.miny
            || j >= self.maxy
            || k < self.minz
            || k >= self.maxz
            || w < self.minw
            || w >= self.maxw
        {
            return None;
        }
        let ywidth = self.maxy - self.miny;
        let zwidth = self.maxz - self.minz;
        let wwidth = self.maxw - self.minw;
        let ind = (i - self.minx) * ywidth * zwidth * wwidth
            + (j - self.miny) * zwidth * wwidth
            + (k - self.minz) * wwidth
            + (w - self.minw);
        Some(ind as usize)
    }

    fn get(&self, i: i32, j: i32, k: i32, w: i32) -> CellState {
        match self.get_ind(i, j, k, w) {
            None => CellState::Inactive,
            Some(ind) => self.data[ind],
        }
    }

    fn set(&mut self, i: i32, j: i32, k: i32, w: i32, v: CellState) {
        match self.get_ind(i, j, k, w) {
            None => panic!("Set on out of bounds index"),
            Some(ind) => self.data[ind] = v,
        }
    }

    fn active_neighbors(&self, i: i32, j: i32, k: i32, w: i32) -> u8 {
        let mut count = 0;

        for di in -1..=1 {
            for dj in -1..=1 {
                for dk in -1..=1 {
                    for dw in -1..=1 {
                        if di == 0 && dj == 0 && dk == 0 && dw == 0 {
                            continue;
                        }
                        if self.get(i + di, j + dj, k + dk, w + dw) == CellState::Active {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }

    fn step(&self) -> Grid4 {
        let mut new_grid = Grid4::empty_expand(self);
        for i in new_grid.minx..new_grid.maxx {
            for j in new_grid.miny..new_grid.maxy {
                for k in new_grid.minz..new_grid.maxz {
                    for w in new_grid.minw..new_grid.maxw {
                        let count = self.active_neighbors(i, j, k, w);
                        match self.get(i, j, k, w) {
                            CellState::Active => {
                                if count == 2 || count == 3 {
                                    new_grid.set(i, j, k, w, CellState::Active)
                                }
                            }
                            CellState::Inactive => {
                                if count == 3 {
                                    new_grid.set(i, j, k, w, CellState::Active)
                                }
                            }
                        }
                    }
                }
            }
        }
        new_grid
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for k in self.minz..self.maxz {
            for i in self.minx..self.maxx {
                for j in self.miny..self.maxy {
                    let char = match self.get(i, j, k) {
                        CellState::Active => "#",
                        CellState::Inactive => ".",
                    };
                    write!(f, "{}", char)?;
                }
                write!(f, "\n")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let init: Vec<_> = stdin()
        .lock()
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| match c {
                    '.' => CellState::Inactive,
                    '#' => CellState::Active,
                    _ => panic!("Invalid character"),
                })
                .collect()
        })
        .collect();
    let mut grid = Grid::new(&init);
    let mut grid4 = Grid4::new(&init);

    for _ in 0..6 {
        grid = grid.step();
        grid4 = grid4.step();
    }
    println!(
        "{}\n{}",
        grid.data
            .iter()
            .filter(|&&x| x == CellState::Active)
            .count(),
        grid4
            .data
            .iter()
            .filter(|&&x| x == CellState::Active)
            .count()
    );
}
