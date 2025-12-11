#[derive(thiserror::Error, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    #[error("Parse error")]
    ParseError,
}

pub type Result<T> = std::result::Result<T, Error>;

fn max_pair(s: &str) -> i32 {
    let (m1, m2) = s.bytes().fold((b'0', b'0'), |(m1, m2), b| {
        if m2 > m1 {
            (m2, b)
        } else {
            if b > m2 {
                (m1, b)
            } else {
                (m1, m2)
            }
        }
    });
    (m1 - b'0') as i32 * 10 + (m2 - b'0') as i32
}

fn reorder(mut v: Vec<u8>) -> std::result::Result<Vec<u8>, Vec<u8>> {
    for i in 1..v.len() {
        if v[i - 1] < v[i] {
            v.remove(i - 1);
            return Ok(v);
        }
    }
    return Err(v);
}

fn max_twelve(s: &str) -> u64 {
    let bytes = s.as_bytes();
    let mut twelve = bytes[..12].to_vec();
    for &b in bytes.iter().skip(12) {
        twelve = match reorder(twelve) {
            Ok(mut changed) => {
                changed.push(b);
                changed
            }
            Err(mut changed) => {
                if let Some(x) = changed.last_mut() {
                    *x = b.max(*x);
                }
                changed
            }
        };
    }
    twelve
        .into_iter()
        .fold(0u64, |acc, b| acc * 10 + (b - b'0') as u64)
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<i32> {
    Ok(lines.iter().map(|line| max_pair(line.as_ref())).sum())
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<u64> {
    Ok(lines.iter().map(|line| max_twelve(line.as_ref())).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA1: &str = r"987654321111111
811111111111119
234234234234278
818181911112111";
    #[test]
    fn task1_test() {
        let lines = DATA1.lines().collect::<Vec<_>>();
        let result = task1(&lines);
        assert_eq!(Ok(357), result);
    }
    #[test]
    fn task2_test() {
        let lines = DATA1.lines().collect::<Vec<_>>();
        let result = task2(&lines);
        assert_eq!(Ok(3121910778619), result);
    }
}
