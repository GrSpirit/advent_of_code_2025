#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Parse error")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("Format error")]
    FormatError,
}

fn traverse(grid: &Vec<Vec<u8>>, i: usize, j: usize, cache: &mut Vec<Vec<u64>>) -> u64 {
    if cache[i][j] != 0 {
        return cache[i][j];
    }
    if i == grid.len() - 1 {
        cache[i][j] = 1;
        return 1;
    }
    if grid[i + 1][j] == b'.' {
        cache[i][j] = traverse(grid, i + 1, j, cache);
        return cache[i][j];
    }
    if grid[i + 1][j] == b'^' {
        cache[i][j] = traverse(grid, i + 1, j - 1, cache) + traverse(grid, i + 1, j + 1, cache);
        return cache[i][j];
    }
    unreachable!("traverse");
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<u32> {
    let mut grid = lines
        .into_iter()
        .map(|l| l.as_ref().bytes().collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    let mut result = 0;
    for i in 1..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'^' {
                if grid[i - 1][j] == b'|' {
                    result += 1;
                    grid[i][j - 1] = b'|';
                    grid[i][j + 1] = b'|';
                }
            } else if grid[i - 1][j] == b'|' || grid[i - 1][j] == b'S' {
                grid[i][j] = b'|';
            }
        }
    }
    Ok(result)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<u64> {
    let grid = lines
        .into_iter()
        .map(|l| l.as_ref().bytes().collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    let mut cache = vec![vec![0; grid[0].len()]; grid.len()];
    let start = grid[0].iter().position(|b| *b == b'S').unwrap();
    Ok(traverse(&grid, 0, start, &mut cache))
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = ".......S.......\n\
    ...............\n\
    .......^.......\n\
    ...............\n\
    ......^.^......\n\
    ...............\n\
    .....^.^.^.....\n\
    ...............\n\
    ....^.^...^....\n\
    ...............\n\
    ...^.^...^.^...\n\
    ...............\n\
    ..^...^.....^..\n\
    ...............\n\
    .^.^.^.^.^...^.\n\
    ...............";
    #[test]
    fn task1_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task1(&lines);
        assert_eq!(Ok(21), result);
    }
    #[test]
    fn task2_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task2(&lines);
        assert_eq!(Ok(40), result);
    }
}
