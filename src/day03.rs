use core::panic;

// Part 1 no regex :)
// State definition:
// error: no match
// ^: Initial state, goes to m
// m: goes to u
// u: goes to l
// l: goes to (
// (: goes to num1
// num1: stays in num1 on digit, goes to ,
// ,: goes to num2
// num2: stays in num2 on digit, goes to )
// ): Final state
#[derive(Debug)]
enum State {
    Initial,
    M,
    U,
    L,
    OpenBracket,
    Num1(usize),
    Comma,
    Num2(usize),
    Final,
    Error,
}

impl State {
    pub fn transition(&self, c: &char) -> Self {
        match self {
            Self::Initial => {
                if *c == 'm' {
                    Self::M
                } else {
                    Self::Error
                }
            }
            Self::M => {
                if *c == 'u' {
                    State::U
                } else {
                    Self::Error
                }
            }
            State::U => {
                if *c == 'l' {
                    State::L
                } else {
                    Self::Error
                }
            }
            State::L => {
                if *c == '(' {
                    State::OpenBracket
                } else {
                    Self::Error
                }
            }
            State::OpenBracket => {
                if let Some(n) = c.to_digit(10) {
                    State::Num1(n as usize)
                } else {
                    Self::Error
                }
            }
            State::Num1(curr) => {
                if let Some(n) = c.to_digit(10) {
                    State::Num1(curr * 10 + n as usize)
                } else if *c == ',' {
                    State::Comma
                } else {
                    Self::Error
                }
            }
            State::Comma => {
                if let Some(n) = c.to_digit(10) {
                    State::Num2(n as usize)
                } else {
                    Self::Error
                }
            }
            State::Num2(curr) => {
                if let Some(n) = c.to_digit(10) {
                    State::Num2(curr * 10 + n as usize)
                } else if *c == ')' {
                    State::Final
                } else {
                    Self::Error
                }
            }
            _ => {
                panic!("Attempted to transition out of final state")
            }
        }
    }
}

#[derive(Debug)]
struct StateMachine {
    state: State,
    size: usize,
    n1: usize,
    n2: usize,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            state: State::Initial,
            size: 0,
            n1: 0,
            n2: 0,
        }
    }

    /// Returns true if transition was successful, false otherwise
    pub fn next_state(&mut self, c: &char) -> bool {
        let new_state = self.state.transition(c);

        // Save n1 or n2
        match (&self.state, &new_state) {
            (State::Num1(n1), State::Comma) => self.n1 = *n1,
            (State::Num2(n2), State::Final) => self.n2 = *n2,
            _ => {}
        }

        self.state = new_state;
        self.size += 1;
        !matches!(self.state, State::Error)
    }

    pub fn is_final(&self) -> bool {
        matches!(self.state, State::Final)
    }
}

pub fn part1(lines: Vec<String>) -> usize {
    // We don't actually want lines, merge the vec back into a string
    let input = lines.join("\n");

    let mut total = 0;
    let mut start_index = 0;

    while start_index < input.len() {
        let mut state_machine = StateMachine::new();
        let mut chars = input.chars().skip(start_index);
        while state_machine.next_state(&chars.next().unwrap()) {
            if !state_machine.is_final() {
                continue;
            }

            total += state_machine.n1 * state_machine.n2;
            start_index += state_machine.size - 1;
            break;
        }
        start_index += 1;
    }

    total
}

// ==============
// === PART 2 ===
// ==============

#[derive(Debug)]
enum Action {
    Mul(usize, usize),
    Do,
    Dont,
}

impl Action {
    /// Returns the number of characters read to parse the Action, as well as the action itself, or
    /// None.
    pub fn try_from_str(s: &str) -> Option<(usize, Self)> {
        if s.starts_with('m') {
            Self::parse_mul(s)
        } else if s.starts_with("do()") {
            Some((4, Action::Do))
        } else if s.starts_with("don't()") {
            Some((7, Action::Dont))
        } else {
            None
        }
    }

    fn parse_mul(s: &str) -> Option<(usize, Self)> {
        let mut chars = s.chars();
        let mut sm = StateMachine::new();
        let mut result = None;
        while chars.next().is_some_and(|c| sm.next_state(&c)) {
            if sm.is_final() {
                result = Some((sm.size, Self::Mul(sm.n1, sm.n2)));
                break;
            }
        }
        result
    }
}

pub fn part2(lines: Vec<String>) -> usize {
    // We don't actually want lines, merge the vec back into a string
    let input = lines.join("\n");

    let mut enabled = true;
    let mut total = 0;
    let mut index = 0;
    while index < input.len() {
        if let Some((n, action)) = Action::try_from_str(&input[index..]) {
            index += n;
            match action {
                Action::Do => enabled = true,
                Action::Dont => enabled = false,
                Action::Mul(n1, n2) if enabled => total += n1 * n2,
                Action::Mul(_, _) => {} // No action if not enabled
            }
        } else {
            index += 1;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = ["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"]
            .map(ToOwned::to_owned)
            .to_vec();

        assert_eq!(part1(data), 161)
    }

    #[test]
    fn test_part2() {
        let data = ["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"]
            .map(ToOwned::to_owned)
            .to_vec();

        assert_eq!(part2(data), 48)
    }
}
