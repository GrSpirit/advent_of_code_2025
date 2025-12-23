use itertools::Itertools;
use std::convert::TryFrom;
use std::collections::VecDeque;

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Parse error")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("Format error")]
    FormatError,
    #[error("No solution found")]
    NoSolution,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum State {
    On = 1,
    Off = 0,
}

impl TryFrom<char> for State {
    type Error = Error;
    fn try_from(c: char) -> Result<Self> {
        match c {
            '#' => Ok(State::On),
            '.' => Ok(State::Off),
            _ => Err(Error::FormatError),
        }
    }
}

fn parse_line(line: &str) -> Result<(Vec<State>, Vec<Vec<usize>>, Vec<i16>)> {
    let mut states: Option<Vec<State>> = None;
    let mut buttons = Vec::new();
    let mut joltage: Option<Vec<i16>> = None;
    for part in line.split_whitespace() {
        if let Some(stripped) = part.strip_prefix("[").and_then(|s| s.strip_suffix("]")) {
            states = Some(stripped.chars().map(|c| State::try_from(c)).collect::<Result<Vec<_>>>()?);
        } else if let Some(stripped) = part.strip_prefix("(").and_then(|s| s.strip_suffix(")")) {
            buttons.push(stripped.split(',').map(|s| s.parse::<usize>().map_err(Error::from)).collect::<Result<Vec<_>>>()?);
        } else if let Some(stripped) = part.strip_prefix("{").and_then(|s| s.strip_suffix("}")) {
            joltage = Some(stripped.split(',').map(|s| s.parse::<i16>().map_err(Error::from)).collect::<Result<Vec<_>>>()?);
        } else {
            return Err(Error::FormatError);
        }
    }
    Ok((states.ok_or(Error::FormatError)?, buttons, joltage.ok_or(Error::FormatError)?))
}

fn parse_input<S: AsRef<str>>(input: &[S]) -> Result<Vec<(Vec<State>, Vec<Vec<usize>>, Vec<i16>)>> {
    input.iter().map(|line| parse_line(line.as_ref())).collect()
}

fn state_to_u32(state: &[State]) -> u32 {
    state.iter().enumerate().fold(0, |acc, (i, state)| acc | (*state as u32) << i)
}

fn buttons_to_u32(buttons: &[usize]) -> u32 {
    buttons.iter().fold(0, |acc, button| acc | (1 << *button))
}

fn bfs(buttons: &[usize], target_joltage: Vec<i16>) -> Result<u32> {
    let mut queue = VecDeque::new();
    queue.push_back((0, target_joltage));
    while let Some((steps, current_joltage)) = queue.pop_front() {
        if current_joltage.iter().all(|a| *a == 0) {
            return Ok(steps);
        }
        for button in buttons {
            let new_joltage = current_joltage.iter().enumerate().map(|(i, joltage)| if (1 << i) & *button != 0 { *joltage - 1 } else { *joltage }).collect::<Vec<_>>();
            if new_joltage.iter().all(|joltage| *joltage >= 0) {
                queue.push_back((steps + 1, new_joltage));
            }
        }
    }
    Err(Error::NoSolution)
}

struct Helper {
    buttons: Vec<usize>,
}

impl Helper {
    fn new(buttons: Vec<usize>) -> Self {
        Self { buttons }
    }

    fn decrease_joltage(joltage: &mut Vec<i16>, button: usize) {
        let mut button = button;
        let mut i = 0;
        while button != 0 {
            if button & 1 == 1 {
                joltage[i] -= 1;
            }
            i += 1;
            button >>= 1;
        }
    }

    fn increase_joltage(joltage: &mut Vec<i16>, button: usize) {
        let mut button = button;
        let mut i = 0;
        while button != 0 {
            if button & 1 == 1 {
                joltage[i] += 1;
            }
            i += 1;
            button >>= 1;
        }
    }

    fn dfs(&self, steps: u32, current_joltage: &mut Vec<i16>) -> u32 {
        if current_joltage.iter().all(|a| *a == 0) {
            return steps;
        }
        let mut min_steps = u32::MAX;
        for button in self.buttons.iter().cloned() {
            Self::decrease_joltage(current_joltage, button);
            if current_joltage.iter().all(|a| *a >= 0) {
                min_steps = min_steps.min(self.dfs(steps + 1, current_joltage));
            }
            Self::increase_joltage(current_joltage, button);
        }
        min_steps
    }
}

pub fn task1<S: AsRef<str>>(input: &[S]) -> Result<i64> {
    let machines = parse_input(input)?;
    let result = machines.iter().map(|(target_state, buttons, _joltage)| {
        let target_state = state_to_u32(target_state);
        let buttons = buttons.iter().map(|buttons| buttons_to_u32(buttons)).collect::<Vec<_>>();
        let mut min_steps = usize::MAX;
        for buttons_permutation in buttons.iter().permutations(buttons.len()) {
            let mut state = 0;
            for (i, button) in buttons_permutation.iter().take(min_steps - 1).enumerate() {
                state ^= **button;
                if state == target_state {
                    min_steps = min_steps.min(i + 1);
                    break;
                }
            }
        }
        min_steps as i64
    }).sum();
    Ok(result)
}

pub fn task2<S: AsRef<str>>(input: &[S]) -> Result<u32> {
    let machines = parse_input(input)?;
    let result = machines.into_iter().map(|(_, buttons, mut joltage)| {
        let buttons = buttons.iter().map(|button| buttons_to_u32(button) as usize).collect::<Vec<_>>();
        // bfs(&buttons, joltage.clone()).unwrap()
        let helper = Helper::new(buttons);
        helper.dfs(0, &mut joltage)
    }).sum();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    #[test]
    fn parse_input_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = parse_input(&lines);
        assert_eq!(Ok(vec![
            (
                vec![State::Off, State::On, State::On, State::Off],
                vec![vec![3], vec![1, 3], vec![2], vec![2, 3], vec![0, 2], vec![0, 1]],
                vec![3, 5, 4, 7]
            ),
            (
                vec![State::Off, State::Off, State::Off, State::On, State::Off],
                vec![vec![0, 2, 3, 4], vec![2, 3], vec![0, 4], vec![0, 1, 2], vec![1, 2, 3, 4]],
                vec![7, 5, 12, 7, 2]
            ),
            (
                vec![State::Off, State::On, State::On, State::On, State::Off, State::On],
                vec![vec![0, 1, 2, 3, 4], vec![0, 3, 4], vec![0, 1, 2, 4, 5], vec![1, 2]],
                vec![10, 11, 11, 5, 10, 5]
            )
        ]), result);
    }
    #[test]
    fn task1_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task1(&lines);
        assert_eq!(Ok(7), result);
    }
    #[test]
    fn task2_test() {
        let lines = DATA.lines().collect::<Vec<_>>();
        let result = task2(&lines);
        assert_eq!(Ok(33), result);
    }
}
