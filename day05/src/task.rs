use std::collections::{HashMap, HashSet};

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Parse error")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("Format error")]
    FormatError,
}

pub type Result<T> = std::result::Result<T, Error>;

fn is_correct_order(nums: &[i32], depends_on: &HashMap<i32, Vec<i32>>) -> bool {
    let mut visited: HashSet<i32> = HashSet::new();
    for x in nums {
        for d in depends_on.get(x).unwrap_or(&Vec::new()) {
            if visited.contains(d) {
                return false;
            }
        }
        visited.insert(*x);
    }
    return true;
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<i32> {
    let split_pos = lines.iter().position(|s| s.as_ref().is_empty()).ok_or(Error::FormatError)?;
    let (pairs, nums) = (&lines[..split_pos], &lines[split_pos + 1..]);
    let mut depends_on: HashMap<i32, Vec<i32>> = HashMap::new();
    for pair in pairs {
        let (a, b) = pair.as_ref().split_once('|').ok_or_else(|| Error::FormatError)?;
        depends_on.entry(a.parse::<i32>()?).or_default().push(b.parse::<i32>()?);
    }
    let pages = nums.iter()
        .map(|s| s.as_ref().split(',').map(|x| x.parse::<i32>().map_err(|e| Error::ParseError(e))).collect::<Result<Vec<_>>>())
        .collect::<Result<Vec<_>>>()?;
    let mut ans = 0;
    for page in pages.iter() {
        if is_correct_order(page, &depends_on) {
            ans += page[page.len() / 2];
        } else {
        }
    }
    Ok(ans)
}

pub fn task2<S: AsRef<str>>(_lines: &[S]) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    #[test]
    fn task1_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task1(&lines);
        assert_eq!(Ok(143), result);
    }
    #[test]
    fn task2_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task2(&lines);
        assert_eq!(Ok(0), result);
    }
}
