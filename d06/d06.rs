use anyhow::anyhow;
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use vecmath::Vector2;

type Pos = Vector2<i32>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Instruction {
    On(Pos, Pos),
    Off(Pos, Pos),
    Toggle(Pos, Pos),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_pos(p: &str) -> Result<Pos, anyhow::Error> {
            let mut it = p.split(',');
            Ok([
                it.next().ok_or_else(|| anyhow!("expected int"))?.parse()?,
                it.next().ok_or_else(|| anyhow!("expected int"))?.parse()?,
            ])
        }
        let mut tokens = s.split(' ');
        let cons = match tokens.next() {
            Some("turn") => match tokens.next() {
                Some("on") => Instruction::On,
                Some("off") => Instruction::Off,
                _ => return Err(anyhow!("expected on|off")),
            },
            Some("toggle") => Instruction::Toggle,
            _ => return Err(anyhow!("expected turn|toggle")),
        };
        let from = parse_pos(tokens.next().ok_or_else(|| anyhow!("expected pos"))?)?;
        if !matches!(tokens.next(), Some("through")) {
            return Err(anyhow!("expected through"));
        }
        let to = parse_pos(tokens.next().ok_or_else(|| anyhow!("expected pos"))?)?;
        Ok(cons(from, to))
    }
}

struct Grid<T>(Vec<T>);

impl Grid<bool> {
    fn new() -> Grid<bool> {
        Grid(vec![false; 1_000_000])
    }
}

impl Grid<u32> {
    fn new() -> Grid<u32> {
        Grid(vec![0; 1_000_000])
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = <Vec<T> as Index<usize>>::Output;

    fn index(&self, index: usize) -> &Self::Output {
        Index::index(&self.0, index)
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        IndexMut::index_mut(&mut self.0, index)
    }
}

fn execute_instruction1(mut grid: Grid<bool>, ins: Instruction) -> Grid<bool> {
    use Instruction::*;
    let (On(from, to) | Off(from, to) | Toggle(from, to)) = ins;
    for y in from[0]..=to[0] {
        for x in from[1]..=to[1] {
            let x = x as usize;
            let y = y as usize;
            match ins {
                On(..) => grid[y * 1000 + x] = true,
                Off(..) => grid[y * 1000 + x] = false,
                Toggle(..) => grid[y * 1000 + x] ^= true,
            };
        }
    }
    grid
}

fn execute_instruction2(mut grid: Grid<u32>, ins: Instruction) -> Grid<u32> {
    use Instruction::*;
    let (On(from, to) | Off(from, to) | Toggle(from, to)) = ins;
    for y in from[0]..=to[0] {
        for x in from[1]..=to[1] {
            let x = x as usize;
            let y = y as usize;
            match ins {
                On(..) => grid[y * 1000 + x] += 1,
                Off(..) => {
                    grid[y * 1000 + x] = if grid[y * 1000 + x] != 0 {
                        grid[y * 1000 + x] - 1
                    } else {
                        grid[y * 1000 + x]
                    }
                }
                Toggle(..) => grid[y * 1000 + x] += 2,
            };
        }
    }
    grid
}

fn main() {
    let input = include_str!("input.txt");
    let result = input
        .lines()
        .map(|line| line.parse().unwrap())
        .fold(Grid::<bool>::new(), execute_instruction1);
    let count = result.0.iter().filter(|&&x| x).count();
    println!("Part1: {}", count);
    let result = input
        .lines()
        .map(|line| line.parse().unwrap())
        .fold(Grid::<u32>::new(), execute_instruction2);
    let count: u32 = result.0.iter().sum();
    println!("Part2: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        assert!(matches!("".parse(), Result::<Instruction, _>::Err(_)));
        assert!(matches!(
            "turn on 0,0 through 999,999".parse(),
            Ok(Instruction::On([0, 0], [999, 999]))
        ));
        assert!(matches!(
            "toggle 0,0 through 999,0".parse(),
            Ok(Instruction::Toggle([0, 0], [999, 0]))
        ));
        assert!(matches!(
            "turn off 499,499 through 500,500".parse(),
            Ok(Instruction::Toggle([499, 499], [500, 500]))
        ));
    }
}
