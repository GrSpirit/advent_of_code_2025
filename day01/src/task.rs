use std::{fmt, str::FromStr};

#[derive(thiserror::Error, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    #[error("Parse error")]
    ParseError,
}

pub type Result<T> = std::result::Result<T, Error>;

enum Command {
    Left(i32),
    Right(i32),
}

impl FromStr for Command {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match (&s[..1], s[1..].parse::<i32>()) {
            ("L", Ok(x)) => Ok(Command::Left(x)),
            ("R", Ok(x)) => Ok(Command::Right(x)),
            _ => Err(Error::ParseError),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Left(x) => write!(f, "L{}", x),
            Command::Right(x) => write!(f, "R{}", x),
        }
    }
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<i32> {
    let mut pos = 50;
    let mut ans = 0;
    let module = 100;
    let commands = lines
        .iter()
        .map(|s| s.as_ref().parse::<Command>())
        .collect::<Result<Vec<Command>>>()?;
    for cmd in commands {
        match cmd {
            Command::Left(n) => {
                pos = (pos - n) % module;
            }
            Command::Right(n) => {
                pos = ((pos + n) % module + module) % module;
            }
        }
        if pos == 0 {
            ans += 1;
        }
    }
    Ok(ans)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<i32> {
    let mut pos = 50;
    let mut ans = 0;
    let module = 100;
    let commands = lines
        .iter()
        .map(|s| s.as_ref().parse::<Command>())
        .collect::<Result<Vec<Command>>>()?;
    for cmd in commands {
        let next_pos = match cmd {
            Command::Left(n) => pos - n,
            Command::Right(n) => pos + n,
        };
        if next_pos == 0 {
            ans += 1;
        } else if next_pos > 99 {
            ans += next_pos / 100;
        } else if next_pos < 0 {
            ans += (next_pos / -100) + if pos == 0 { 0 } else { 1 };
        }
        pos = (next_pos % module + module) % module;
        // println!(
        //     "cmd={}, next_pos={}, pos={}, ans={}",
        //     cmd, next_pos, pos, ans
        // );
    }
    Ok(ans)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    #[test]
    fn task1_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task1(&lines);
        assert_eq!(Ok(3), result);
    }
    #[test]
    fn task2_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task2(&lines);
        assert_eq!(Ok(6), result);
    }
}
