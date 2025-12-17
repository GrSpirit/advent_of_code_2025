use std::collections::{BinaryHeap, HashSet};

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Parse error")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("Format error")]
    FormatError,
}

pub type Result<T> = std::result::Result<T, Error>;

fn distance(a: &(i64, i64, i64), b: &(i64, i64, i64)) -> i64 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)
}

pub fn task1<S: AsRef<str>>(lines: &[S], mut count: u32) -> Result<u32> {
    fn find_circuite(circuites: &Vec<HashSet<usize>>, key: usize) -> Option<usize> {
        circuites.iter().position(|set| set.contains(&key))
    }
    let points = lines
        .iter()
        .map(|l| {
            let v = l
                .as_ref()
                .split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (v[0], v[1], v[2])
        })
        .collect::<Vec<(i64, i64, i64)>>();
    let n = points.len();
    let mut distances: BinaryHeap<(i64, usize, usize)> = BinaryHeap::new();
    for i in 0..n {
        for j in (i + 1)..n {
            distances.push((-distance(&points[i], &points[j]), i, j));
        }
    }
    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    while let Some((d, i, j)) = distances.pop() {
        let id1 = find_circuite(&circuits, i);
        let id2 = find_circuite(&circuits, j);
        match (id1, id2) {
            (Some(s1), Some(s2)) if s1 == s2 => {}
            (Some(s1), Some(s2)) => {
                circuits[s1] = circuits[s1].union(&circuits[s2]).cloned().collect();
                circuits.remove(s2);
            }
            (Some(s), None) => {
                circuits[s].insert(j);
            }
            (None, Some(s)) => {
                circuits[s].insert(i);
            }
            (None, None) => {
                circuits.push(HashSet::from([i, j]));
            }
        }
        count -= 1;
        if count == 0 {
            break;
        }
    }
    circuits.sort_by_key(|c| std::cmp::Reverse(c.len()));
    Ok(circuits.iter().take(3).map(|c| c.len()).product::<usize>() as _)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<i64> {
    fn find_circuite(circuites: &Vec<HashSet<usize>>, key: usize) -> Option<usize> {
        circuites.iter().position(|set| set.contains(&key))
    }
    let points = lines
        .iter()
        .map(|l| {
            let v = l
                .as_ref()
                .split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (v[0], v[1], v[2])
        })
        .collect::<Vec<(i64, i64, i64)>>();
    let n = points.len();
    let mut distances: BinaryHeap<(i64, usize, usize)> = BinaryHeap::new();
    for i in 0..n {
        for j in (i + 1)..n {
            distances.push((-distance(&points[i], &points[j]), i, j));
        }
    }
    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    let mut ans = 0;
    while let Some((_, i, j)) = distances.pop() {
        let id1 = find_circuite(&circuits, i);
        let id2 = find_circuite(&circuits, j);
        match (id1, id2) {
            (Some(s1), Some(s2)) if s1 == s2 => {}
            (Some(s1), Some(s2)) => {
                ans = points[i].0 * points[j].0;
                circuits[s1] = circuits[s1].union(&circuits[s2]).cloned().collect();
                circuits.remove(s2);
            }
            (Some(s), None) => {
                ans = points[i].0 * points[j].0;
                circuits[s].insert(j);
            }
            (None, Some(s)) => {
                ans = points[i].0 * points[j].0;
                circuits[s].insert(i);
            }
            (None, None) => {
                ans = points[i].0 * points[j].0;
                circuits.push(HashSet::from([i, j]));
            }
        }
    }
    Ok(ans)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    #[test]
    fn task1_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task1(&lines, 10);
        assert_eq!(Ok(40), result);
    }
    #[test]
    fn task2_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task2(&lines);
        assert_eq!(Ok(25272), result);
    }
}
