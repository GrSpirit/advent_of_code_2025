#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Parse error")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("Format error")]
    FormatError,
}

pub type Result<T> = std::result::Result<T, Error>;

fn transpose<S: AsRef<str>>(matrix: &[S]) -> Vec<Vec<u8>> {
    let n = matrix.len();
    let m = matrix[0].as_ref().len();
    let mut result = vec![vec![b' '; n]; m];
    for i in 0..n {
        for (j, b) in matrix[i].as_ref().bytes().enumerate() {
            result[j][i] = b;
        }
    }
    result
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<u64> {
    let split_pos = lines
        .iter()
        .position(|s| s.as_ref().starts_with('*'))
        .ok_or(Error::FormatError)?;
    let nums = lines[..split_pos]
        .iter()
        .map(|s| {
            s.as_ref()
                .split_ascii_whitespace()
                .map(|ss| ss.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();
    let commands = lines[split_pos]
        .as_ref()
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();
    let mut result = 0u64;
    for (i, &cmd) in commands.iter().enumerate() {
        result += match cmd {
            "+" => nums.iter().fold(0u64, |acc, v| acc + v[i]),
            "*" => nums.iter().fold(1u64, |acc, v| acc * v[i]),
            _ => 0,
        }
    }
    Ok(result)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Plus,
    Mult,
}

fn parse(s: &Vec<u8>) -> Option<(u64, Option<Operator>)> {
    if s.is_empty() || s.iter().all(|c| *c == b' ') {
        return None;
    }
    let mut num = 0u64;
    let mut op = None;
    for b in s {
        match b {
            b'+' => {
                op = Some(Operator::Plus);
            }
            b'*' => {
                op = Some(Operator::Mult);
            }
            n if n.is_ascii_digit() => {
                num = num * 10 + (n - b'0') as u64;
            }
            _ => {}
        }
    }
    Some((num, op))
}

fn compute(nums: &Vec<u64>, operation: Operator) -> u64 {
    match operation {
        Operator::Plus => nums.iter().sum(),
        Operator::Mult => nums.iter().product(),
    }
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<u64> {
    let lines = transpose(lines);
    let mut last_op = None;
    let mut nums = Vec::new();
    let mut result = 0;
    for l in lines {
        match parse(&l) {
            None => {
                result += compute(&nums, last_op.unwrap());
                nums.clear();
                last_op = None;
            }
            Some((x, maybe_op)) => {
                nums.push(x);
                if let Some(op) = maybe_op {
                    last_op = Some(op);
                }
            }
        }
    }
    result += compute(&nums, last_op.unwrap());
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
    #[test]
    fn task1_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task1(&lines);
        assert_eq!(Ok(4277556), result);
    }
    #[test]
    fn task2_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task2(&lines);
        assert_eq!(Ok(3263827), result);
    }
}
