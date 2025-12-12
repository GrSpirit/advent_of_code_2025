use std::mem;

#[derive(thiserror::Error, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    #[error("Parse error")]
    ParseError,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Cell {
    Empty,
    Roll,
}

fn parse_cell(c: char) -> Result<Cell> {
    match c {
        '.' => Ok(Cell::Empty),
        '@' => Ok(Cell::Roll),
        _ => Err(Error::ParseError),
    }
}

fn count_rolls(grid: &Vec<Vec<Cell>>, i: usize, j: usize) -> u8 {
    let start_i = i.checked_sub(1).unwrap_or(0);
    let end_i = grid.len().min(i + 2);
    let start_j = j.checked_sub(1).unwrap_or(0);
    let end_j = grid[0].len().min(j + 2);
    let mut result = 0;
    for ii in start_i..end_i {
        for jj in start_j..end_j {
            if ii == i && jj == j {
                continue;
            }
            if grid[ii][jj] == Cell::Roll {
                result += 1;
            }
        }
    }
    result
}

pub fn task1<S: AsRef<str>>(lines: &[S]) -> Result<i32> {
    let grid = lines
        .into_iter()
        .map(|line| {
            line.as_ref()
                .chars()
                .map(parse_cell)
                .collect::<Result<Vec<Cell>>>()
        })
        .collect::<Result<Vec<Vec<Cell>>>>()?;
    let (n, m) = (grid.len(), grid[0].len());
    let mut count = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == Cell::Roll && count_rolls(&grid, i, j) < 4 {
                count += 1;
            }
        }
    }
    Ok(count)
}

pub fn task2<S: AsRef<str>>(lines: &[S]) -> Result<i32> {
    let mut grid = lines
        .into_iter()
        .map(|line| {
            line.as_ref()
                .chars()
                .map(parse_cell)
                .collect::<Result<Vec<Cell>>>()
        })
        .collect::<Result<Vec<Vec<Cell>>>>()?;
    let (n, m) = (grid.len(), grid[0].len());
    let mut new_grid = vec![vec![Cell::Empty; m]; n];
    let mut count = 0;
    let mut is_changed = true;
    while is_changed {
        is_changed = false;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == Cell::Roll {
                    if count_rolls(&grid, i, j) < 4 {
                        count += 1;
                        is_changed = true;
                        new_grid[i][j] = Cell::Empty;
                    } else {
                        new_grid[i][j] = Cell::Roll;
                    }
                } else {
                    new_grid[i][j] = Cell::Empty;
                }
            }
        }
        mem::swap(&mut grid, &mut new_grid);
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    #[test]
    fn task1_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task1(&lines);
        assert_eq!(Ok(13), result);
    }
    #[test]
    fn task2_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task2(&lines);
        assert_eq!(Ok(43), result);
    }
}
