advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left:Vec<i32> = Vec::new();
    let mut right:Vec<i32> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut words = line.split(' ');
        left.push(words.next().unwrap().parse::<i32>().expect("left msg"));
        words.next(); //throw away space (but there's two?)
        words.next();
        right.push(words.next().unwrap().parse::<i32>().expect("right msg"));
    }

    left.sort();
    right.sort();

    let mut total: u32 = 0;
    let mut i = 0;
    for entry in left.iter() {
        total += entry.abs_diff(right[i]);
        i +=1;
    }

    Some(total)
}

use std::{collections::HashMap, ops::Mul};
pub fn part_two(input: &str) -> Option<u32> {
    let mut left:Vec<u32> = Vec::new();
    let mut right:HashMap<u32, u32> = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut words = line.split(' ');
        left.push(words.next().unwrap().parse::<u32>().expect("left 2 msg"));
        words.next(); //throw away space (but there's two?)
        words.next();

        let parsed = words.next().unwrap().parse::<u32>().expect("right 2 msg");
        let prev_value = right.get(&parsed).copied().unwrap_or(0);
        right.insert(parsed, prev_value+1);
    }

    left.sort();

    let mut total = 0;
    for entry in left.iter() {
        total += entry.mul(right.get(&entry).copied().unwrap_or(0));
    }
    Some(total)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
