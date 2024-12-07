use std::collections::HashMap;

pub fn part1(lines: Vec<String>) -> usize {
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];

    for line in lines {
        let mut nums = line.split_whitespace();
        left.push(nums.next().unwrap().parse().unwrap());
        right.push(nums.next().unwrap().parse().unwrap());
    }

    left.sort();
    right.sort();
    let mut total = 0;
    for (l, r) in left.iter().zip(right) {
        total += l.abs_diff(r);
    }
    total
}

pub fn part2(lines: Vec<String>) -> usize {
    let mut left: Vec<usize> = vec![];
    let mut right_dict: HashMap<usize, usize> = HashMap::new();

    for line in lines {
        let mut nums = line.split_whitespace();

        left.push(nums.next().unwrap().parse::<usize>().unwrap());

        let right = nums.next().unwrap().parse::<usize>().unwrap();
        match right_dict.get_mut(&right) {
            Some(v) => *v += 1,
            None => _ = right_dict.insert(right, 1),
        };
    }

    let mut total = 0;
    for n in left {
        total += n * right_dict.get(&n).unwrap_or(&0);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = ["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"]
            .map(ToOwned::to_owned)
            .to_vec();

        assert_eq!(part1(data), 11)
    }

    #[test]
    fn test_part2() {
        let data = ["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"]
            .map(ToOwned::to_owned)
            .to_vec();

        assert_eq!(part2(data), 31)
    }
}
