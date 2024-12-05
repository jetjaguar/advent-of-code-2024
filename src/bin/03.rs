advent_of_code::solution!(3);

#[derive(Debug)]
struct MulPair {
    value1: u32,
    value2: u32,
}

use regex::Regex;
pub fn part_one(input: &str) -> Option<u32> {
    
    let mut my_mul_pairs: Vec<MulPair> = Vec::new();
    
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        for capture in Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap().find_iter(line) {
            if let  Some(inner_captures) = Regex::new(r"mul\((\d+),(\d+)\)").unwrap().captures(capture.as_str()) 
            {
                my_mul_pairs.push(MulPair {
                    value1: inner_captures[1].parse::<u32>().unwrap(),
                    value2: inner_captures[2].parse::<u32>().unwrap() 
                });
            }            
        }
    }

    //println!("MulPairs {:?}", my_mul_pairs);
    let mut total: u32 = 0;
    for pair in my_mul_pairs.iter()
    {
        total += pair.value1 * pair.value2;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {

    let prepend = String::from("do()");
    let mut prependedinput = String::with_capacity(prepend.len() + input.len());
    prependedinput.push_str(&prepend);
    prependedinput.push_str(input);

    //println!("input {}", input);
    //println!("preinput {}", prependedinput);

    let mut my_mul_pairs: Vec<MulPair> = Vec::new();
    let mut toggle = true;
    for line in prependedinput.lines() {
        if line.is_empty() {
            continue;
        }

        for capture in Regex::new(r"do\(\)|don't\(\)|mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap().find_iter(line) {
            //println!("toggle in {}, capture {}", toggle, capture.as_str());
            if capture.as_str().contains("don't()") {
                toggle = false;
            } else {
                if capture.as_str().contains("do()")
                {
                    toggle = true;
                }
            }
            if toggle {
                if let Some(inner_captures) = Regex::new(r"mul\((\d+),(\d+)\)").unwrap().captures(capture.as_str())
                {
                    my_mul_pairs.push(MulPair {
                        value1: inner_captures[1].parse::<u32>().unwrap(),
                        value2: inner_captures[2].parse::<u32>().unwrap() 
                    });
                }
            }       
        }
    }

    //println!("MulPairs {:?}", my_mul_pairs);
    let mut total: u32 = 0;
    for pair in my_mul_pairs.iter()
    {
        total += pair.value1 * pair.value2;
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
