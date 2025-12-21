use itertools::Itertools;
use glam::I64Vec2;

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Parse error")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("Format error")]
    FormatError,
}

pub type Result<T> = std::result::Result<T, Error>;

fn square(a: &I64Vec2, b: &I64Vec2) -> i64 {
    ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1)
}

fn parse_point<S: AsRef<str>>(line: &S) -> Result<I64Vec2> {
    let (x, y) = line.as_ref().split_once(',').ok_or(Error::FormatError)?;
    Ok(I64Vec2::new(x.parse::<i64>()?, y.parse::<i64>()?))
}

fn parse_input<S: AsRef<str>>(lines: &[S]) -> Result<Vec<I64Vec2>> {
    lines.iter().map(parse_point).collect()
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<i64> {
    let points = parse_input(lines)?;
    let max_square = points.iter()
        .tuple_combinations()
        .map(|(a, b)| square(a, b))
        .max()
        .ok_or(Error::FormatError)?;
    Ok(max_square)
}

pub fn task2<S: AsRef<str>>(input: &[S]) -> Result<i64> {
    fn is_valid(line: &(I64Vec2, I64Vec2), a: &I64Vec2, b: &I64Vec2) -> bool {
        let (line_start, line_end) = line;
        let left = a.x.max(b.x) <= line_start.x.min(line_end.x);
        let right = a.x.min(b.x) >= line_start.x.max(line_end.x);
        let top = a.y.max(b.y) <= line_start.y.min(line_end.y);
        let bottom = a.y.min(b.y) >= line_start.y.max(line_end.y);
        left || right || top || bottom
    }
    let points = parse_input(input)?;
    let lines = points.iter()
        .cloned()
        .circular_tuple_windows()
        .collect::<Vec<(I64Vec2, I64Vec2)>>();
    let max_box = points.iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, square(a, b)))
        .sorted_unstable_by_key(|(_, _, square)| *square)
        .rev()
        .find(|(a, b, _)| {
            lines.iter().all(|line| is_valid(line, a, b))
        })
        .ok_or(Error::FormatError)?;
    Ok(max_box.2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    #[test]
    fn task1_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task1(&lines);
        assert_eq!(Ok(50), result);
    }
    #[test]
    fn task2_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task2(&lines);
        assert_eq!(Ok(24), result);
    }
}
