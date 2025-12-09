#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Parse int error")]
    ParseIntError(#[from] std::num::ParseIntError),
}

pub type Result<T> = std::result::Result<T, Error>;

fn half(x: u64) -> Result<u64> {
    let s = format!("{}", x);
    let n = s.len() / 2;
    if n == 0 {
        return Ok(1);
    }
    Ok(s[..n].parse()?)
}

fn half_one(x: u64) -> Result<u64> {
    let s = format!("{}", x);
    let n = s.len() / 2 + s.len() % 2;
    Ok(s[..n].parse()?)
}

fn has_substr(s: &str, sub: &str) -> bool {
    s.len() == 0 || (s.starts_with(sub) && has_substr(&s[sub.len()..], sub))
}

fn is_valid(s: &str) -> bool {
    let n = s.len() / 2;
    for i in 1..=n {
        if s.len() % i != 0 {
            continue;
        }
        if has_substr(&s[i..], &s[..i]) {
            return true;
        }
    }
    return false;
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<u64> {
    let mut ans = 0u64;
    let ranges = lines[0]
        .as_ref()
        .split(',')
        .map(|s| s.split_once('-').unwrap());
    for (start, end) in ranges {
        let start_num = start.parse::<u64>()?;
        let end_num = end.parse::<u64>()?;
        let start_x = half(start_num)?;
        let end_x = half_one(end_num)?;
        for x in start_x..=end_x {
            let prop = format!("{}{}", x, x).parse::<u64>()?;
            if prop >= start_num && prop <= end_num {
                ans += prop;
            }
        }
    }
    Ok(ans)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<u64> {
    let mut ans = 0u64;
    let ranges = lines[0]
        .as_ref()
        .split(',')
        .map(|s| s.split_once('-').unwrap());
    for (start, end) in ranges {
        let start_num = start.parse::<u64>()?;
        let end_num = end.parse::<u64>()?;
        for x in start_num..=end_num {
            if is_valid(&format!("{}", x)) {
                ans += x;
            }
        }
    }
    Ok(ans)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str =
"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    #[test]
    fn task1_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task1(&lines);
        assert_eq!(Ok(1227775554), result);
    }
    #[test]
    fn task2_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task2(&lines);
        assert_eq!(Ok(4174379265), result);
    }
    #[test]
    fn task1_test21() {
        let lines = vec!["1-21"];
        assert_eq!(Ok(11), task1(&lines));
    }
}
