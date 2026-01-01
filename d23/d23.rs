#![feature(assert_matches)]

use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, i32, space0, space1};
use nom::combinator::{eof, map, value, verify};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

type Reg = char;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Ins {
    /// Half the contents of register r
    Hlf(Reg),
    /// Triple the contents of register r
    Tpl(Reg),
    /// Increment the contents of register r by 1
    Inc(Reg),
    /// Jump to offset o
    Jmp(i32),
    /// Jump to offset o if register r is even
    Jie(Reg, i32),
    /// Jump to offset o if register r is 1
    Jio(Reg, i32),
}

impl fmt::Display for Ins {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Ins::Hlf(r) => write!(f, "hlf {}", r),
            Ins::Tpl(r) => write!(f, "tpl {}", r),
            Ins::Inc(r) => write!(f, "inc {}", r),
            Ins::Jmp(o) => write!(f, "jmp {:+}", o),
            Ins::Jie(r, o) => write!(f, "jie {}, {:+}", r, o),
            Ins::Jio(r, o) => write!(f, "jio {}, {:+}", r, o),
        }
    }
}

impl FromStr for Ins {
    type Err = nom::Err<nom::error::Error<String>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        alt((
            map(
                (
                    alt((
                        value(Ins::Hlf as fn(Reg) -> Ins, tag("hlf")),
                        value(Ins::Tpl as fn(Reg) -> Ins, tag("tpl")),
                        value(Ins::Inc as fn(Reg) -> Ins, tag("inc")),
                    )),
                    space1,
                    verify(anychar, |c| c.is_alphabetic()),
                    eof,
                ),
                |(ins, _, r, _)| ins(r),
            ),
            map((tag("jmp"), space1, i32, eof), |(_, _, o, _)| Ins::Jmp(o)),
            map(
                (
                    alt((
                        value(Ins::Jie as fn(Reg, i32) -> Ins, tag("jie")),
                        value(Ins::Jio as fn(Reg, i32) -> Ins, tag("jio")),
                    )),
                    space1,
                    verify(anychar, |c| c.is_alphabetic()),
                    space0,
                    char(','),
                    space0,
                    i32,
                    eof,
                ),
                |(ins, _, r, _, _, _, o, _)| ins(r, o),
            ),
        ))
        .parse(s)
        .map(|(_, o)| o)
        .map_err(<nom::Err<nom::error::Error<&str>>>::to_owned)
    }
}

fn parse_program(s: &str) -> Result<Vec<Ins>, nom::Err<nom::error::Error<String>>> {
    s.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::parse)
        .collect()
}

type Val = i64;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Cpu {
    program: Vec<Ins>,
    pc: i32,
    registers: HashMap<Reg, Val>,
}

impl FromStr for Cpu {
    type Err = nom::Err<nom::error::Error<String>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Cpu {
            program: parse_program(s)?,
            pc: 0,
            registers: HashMap::new(),
        })
    }
}

impl Cpu {
    fn read(&self, r: Reg) -> Val {
        self.registers.get(&r).copied().unwrap_or_default()
    }

    fn write(&mut self, r: Reg, v: Val) {
        self.registers.insert(r, v);
    }
}

fn step(mut cpu: Cpu) -> (Cpu, bool) {
    if cpu.pc < 0 || cpu.pc >= cpu.program.len() as i32 {
        return (cpu, true);
    }
    let ins = cpu.program[cpu.pc as usize];
    cpu.pc += 1;
    match ins {
        Ins::Hlf(r) => cpu.write(r, cpu.read(r) / 2),
        Ins::Tpl(r) => cpu.write(r, cpu.read(r) * 3),
        Ins::Inc(r) => cpu.write(r, cpu.read(r) + 1),
        Ins::Jmp(o) => cpu.pc += o - 1,
        Ins::Jie(r, o) => {
            if cpu.read(r) & 1 == 0 {
                cpu.pc += o - 1
            }
        }
        Ins::Jio(r, o) => {
            if cpu.read(r) == 1 {
                cpu.pc += o - 1
            }
        }
    }
    (cpu, false)
}

fn run(mut cpu: Cpu) -> Cpu {
    loop {
        let (cpu_next, halted) = step(cpu);
        if halted {
            break cpu_next;
        }
        cpu = cpu_next;
    }
}

fn main() {
    let cpu: Cpu = include_str!("input.asm").parse().unwrap();

    let result = run(cpu.clone());
    println!("Part1: b = {}", result.read('b'));

    let mut cpu = cpu;
    cpu.write('a', 1);
    let result = run(cpu);
    println!("Part2: b = {}", result.read('b'));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;

    #[test]
    fn test_parse() {
        assert_eq!("hlf a".parse(), Ok(Ins::Hlf('a')));
        assert_eq!("tpl b".parse(), Ok(Ins::Tpl('b')));
        assert_eq!("inc c".parse(), Ok(Ins::Inc('c')));
        assert_eq!("jmp +42".parse(), Ok(Ins::Jmp(42)));
        assert_eq!("jmp -7".parse(), Ok(Ins::Jmp(-7)));
        assert_eq!("jie a, +10".parse(), Ok(Ins::Jie('a', 10)));
        assert_eq!("jio b, -3".parse(), Ok(Ins::Jio('b', -3)));

        assert_matches!("hlf 1".parse::<Ins>(), Err(_));
        assert_matches!("jmp abc".parse::<Ins>(), Err(_));
    }

    #[test]
    fn test_parse_program() {
        let program = "\
            inc a\n\
            jio a, +2\n\
            tpl a\n\
            inc a\
        ";
        let expected = vec![
            Ins::Inc('a'),
            Ins::Jio('a', 2),
            Ins::Tpl('a'),
            Ins::Inc('a'),
        ];
        assert_eq!(parse_program(program), Ok(expected));
    }

    #[test]
    fn test_step() {
        let cpu = Cpu {
            program: vec![
                Ins::Inc('a'),
                Ins::Jio('a', 2),
                Ins::Tpl('a'),
                Ins::Inc('a'),
            ],
            ..Cpu::default()
        };

        let (cpu, halted) = step(cpu);
        assert!(!halted);
        assert_eq!(cpu.pc, 1);
        assert_eq!(cpu.read('a'), 1);

        let (cpu, halted) = step(cpu);
        assert!(!halted);
        assert_eq!(cpu.pc, 3);
        assert_eq!(cpu.read('a'), 1);

        let (cpu, halted) = step(cpu);
        assert!(!halted);
        assert_eq!(cpu.pc, 4);
        assert_eq!(cpu.read('a'), 2);

        let (cpu, halted) = step(cpu);
        assert!(halted);
        assert_eq!(cpu.pc, 4);
        assert_eq!(cpu.read('a'), 2);
    }

    #[test]
    fn test_run() {
        let cpu = Cpu::from_str(
            r#"
                inc a
                jio a, +2
                tpl a
                inc a
            "#,
        )
        .unwrap();
        let expected = Cpu {
            program: cpu.program.clone(),
            pc: 4,
            registers: HashMap::from([('a', 2)]),
        };
        assert_eq!(run(cpu), expected);
    }
}
