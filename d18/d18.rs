use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
struct Grid {
    data: Vec<bool>,
    width: i32,
    height: i32,
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Grid::default();
        for (i, line) in s.lines().filter(|l| !l.trim().is_empty()).enumerate() {
            let line = line.trim();
            if i == 0 {
                grid.width = line.len() as i32;
            } else if grid.width != line.len() as i32 {
                return Err("line lengths not matching".to_string());
            }
            grid.height = i as i32 + 1;
            grid.data.reserve(line.len());
            for c in line.chars() {
                grid.data.push(match c {
                    '#' => true,
                    '.' => false,
                    _ => return Err(format!("Expected '#' or '.', got {c}")),
                });
            }
        }
        Ok(grid)
    }
}

impl Grid {
    fn idx(&self, x: i32, y: i32) -> usize {
        y as usize * self.width as usize + x as usize
    }

    fn get(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            false
        } else {
            self.data[self.idx(x, y)]
        }
    }

    fn set(&mut self, x: i32, y: i32, v: bool) {
        if x < 0 || x >= self.width || y < 0 || y > self.height {
            panic!("x or y out of bounds!");
        }
        let i = self.idx(x, y);
        self.data[i] = v
    }
}

fn neighbor_count(grid: &Grid, x: i32, y: i32) -> i32 {
    (-1..=1)
        .flat_map(|y_off| {
            (-1..=1)
                .map(move |x_off| (x_off, y_off))
                .filter(|(x_off, y_off)| *x_off != 0 || *y_off != 0)
        })
        .filter(|(x_off, y_off)| grid.get(x + x_off, y + y_off))
        .count() as i32
}

fn conways_game_of_life(grid: Grid) -> Grid {
    let mut result = grid.clone();
    for y in 0..grid.height {
        for x in 0..grid.width {
            result.set(
                x,
                y,
                matches!(
                    (grid.get(x, y), neighbor_count(&grid, x, y)),
                    (true, 2 | 3) | (false, 3)
                ),
            );
        }
    }
    result
}

fn turn_corners_on(mut grid: Grid) -> Grid {
    if grid.width == 0 || grid.height == 0 {
        grid
    } else {
        grid.set(0, 0, true);
        grid.set(grid.width - 1, 0, true);
        grid.set(0, grid.height - 1, true);
        grid.set(grid.width - 1, grid.height - 1, true);
        grid
    }
}

fn main() {
    let grid: Grid = include_str!("input.txt").parse().unwrap();
    let result = (0..100).fold(grid.clone(), |g, _| conways_game_of_life(g));
    let count = result.data.iter().filter(|b| **b).count();
    println!("Part1: {}", count);
    let grid = turn_corners_on(grid);
    let result = (0..100).fold(grid, |g, _| turn_corners_on(conways_game_of_life(g)));
    let count = result.data.iter().filter(|b| **b).count();
    println!("Part2: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            "#.#\n.#.\n..#".parse(),
            Ok(Grid {
                data: vec![true, false, true, false, true, false, false, false, true],
                width: 3,
                height: 3,
            })
        )
    }

    #[test]
    fn test_neighbor_count() {
        let grid = Grid {
            data: vec![true],
            width: 1,
            height: 1,
        };
        assert_eq!(neighbor_count(&grid, 0, 0), 0);

        let grid = Grid {
            data: vec![false, false, false, false, true, false, false, false, false],
            width: 3,
            height: 3,
        };
        assert_eq!(neighbor_count(&grid, 1, 1), 0);
        assert_eq!(neighbor_count(&grid, 0, 0), 1);

        let grid = Grid {
            data: vec![true, false, false, true, true, false, false, true, false],
            width: 3,
            height: 3,
        };
        assert_eq!(neighbor_count(&grid, 1, 1), 3);
        assert_eq!(neighbor_count(&grid, 0, 0), 2);
    }

    #[test]
    fn test_gol_step() {
        let a = "
                .#.#.#
                ...##.
                #....#
                ..#...
                #.#..#
                ####..  "
            .parse()
            .unwrap();
        let b = "
                ..##..
                ..##.#
                ...##.
                ......
                #.....
                #.##..  "
            .parse()
            .unwrap();
        assert_eq!(conways_game_of_life(a), b);
    }

    #[test]
    fn test_gol() {
        let a = "
                .#.#.#
                ...##.
                #....#
                ..#...
                #.#..#
                ####..  "
            .parse()
            .unwrap();
        let b = "
                ......
                ......
                ..##..
                ..##..
                ......
                ......  "
            .parse()
            .unwrap();
        assert_eq!((0..4).fold(a, |g, _| conways_game_of_life(g)), b);
    }
}
