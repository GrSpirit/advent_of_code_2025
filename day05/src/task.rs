#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Parse error")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("Format error")]
    FormatError,
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<i32> {
    let split_pos = lines
        .iter()
        .position(|s| s.as_ref().is_empty())
        .ok_or(Error::FormatError)?;
    let (pairs, nums) = (&lines[..split_pos], &lines[split_pos + 1..]);
    let mut ranges = pairs
        .iter()
        .map(|pair| {
            let p = pair.as_ref().split_once('-').unwrap();
            (p.0.parse::<u64>().unwrap(), p.1.parse::<u64>().unwrap())
        })
        .collect::<Vec<(u64, u64)>>();
    ranges.sort();
    let result = nums
        .into_iter()
        .map(|s| s.as_ref().parse::<u64>().unwrap())
        .filter(|u| ranges.iter().any(|(start, end)| start <= u && u <= end))
        .count();
    Ok(result as _)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<u64> {
    let split_pos = lines
        .iter()
        .position(|s| s.as_ref().is_empty())
        .ok_or(Error::FormatError)?;
    let pairs = &lines[..split_pos];
    let mut ranges = pairs
        .iter()
        .map(|pair| {
            let p = pair.as_ref().split_once('-').unwrap();
            (p.0.parse::<u64>().unwrap(), p.1.parse::<u64>().unwrap())
        })
        .collect::<Vec<(u64, u64)>>();
    ranges.sort();
    let mut unique_ranges: Vec<(u64, u64)> = Vec::new();
    for r in ranges {
        if let Some(last) = unique_ranges.pop() {
            if last.1 >= r.0 {
                if last.1 <= r.1 {
                    unique_ranges.push((last.0, r.1));
                } else {
                    unique_ranges.push(last);
                }
            } else {
                unique_ranges.push(last);
                unique_ranges.push(r);
            }
        } else {
            unique_ranges.push(r);
        }
    }
    let result = unique_ranges
        .into_iter()
        .fold(0, |acc, r| acc + r.1 - r.0 + 1);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32";
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
        assert_eq!(Ok(14), result);
    }
}
