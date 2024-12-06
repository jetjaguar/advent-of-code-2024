advent_of_code::solution!(5);

use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    
    let mut result = 0;
    
    //Read Rules
    let mut rule_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut switch_to_updates = false;
    for line in input.lines() {
                
        if !switch_to_updates && line.is_empty() {
            switch_to_updates = true;
//            println!("map {:?}", rule_map);
        } else if switch_to_updates && line.is_empty() {
            continue;
        } else if !switch_to_updates
        {
            if let Some(captures) = Regex::new(r"(\d+)\|(\d+)").unwrap().captures(line) 
            {
                //Had some difficulty here because I was stomping previous entrys, so need a 2d solution
                rule_map.entry(captures[1].parse::<u32>().unwrap()).or_default().insert(captures[2].parse::<u32>().unwrap());
            }
         //Read updates section
        } else if switch_to_updates
        {
            //Parse update section and check rule
            let updates: Vec<u32> = line.split(',').filter_map(|s| s.parse().ok()).collect();
            let upd_search = updates.clone();
//            println!("update: {:?}", updates);

            let mut i = 0;
            let mut failed_match = false;
            for page in updates.clone().into_iter()
            {
                let rule_set = rule_map.entry(page).or_default();
                for rule_entry in rule_set.clone().into_iter()
                {
                    match upd_search.iter().position(|&r| r == rule_entry)
                    {
                        Some(position) =>
                        {
                            if position < i
                            {
                                failed_match = true;
                                continue;
                            }  
                        } 
                        None => { }
                    }
                }
                i += 1;
            }
            //If correct determine the middle page number
            if !failed_match
            {
                result += updates[updates.len()/2];
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;
    
    //Read Rules
    let mut rule_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut switch_to_updates = false;
    for line in input.lines() {
                
        if !switch_to_updates && line.is_empty() {
            switch_to_updates = true;
//            println!("map {:?}", rule_map);
        } else if switch_to_updates && line.is_empty() {
            continue;
        } else if !switch_to_updates
        {
            if let Some(captures) = Regex::new(r"(\d+)\|(\d+)").unwrap().captures(line) 
            {
                //Had some difficulty here because I was stomping previous entrys, so need a 2d solution
                rule_map.entry(captures[1].parse::<u32>().unwrap()).or_default().insert(captures[2].parse::<u32>().unwrap());
            }
         //Read updates section
        } else if switch_to_updates
        {
            //Parse update section and check rule
            let mut updates: Vec<u32> = line.split(',').filter_map(|s| s.parse().ok()).collect();
            println!("update: {:?}", updates);

            let mut was_incorrectly_ordered = false;
            let mut i = 0;
            while i < updates.len()
            {
                let page = updates[i];
                let rule_set = rule_map.entry(page).or_default();
                for rule_entry in rule_set.clone().into_iter()
                {
                    let mut reset_this_loop = false;
                    let mut j = 0;
                    while j < updates.len()
                    {
                        if updates[j].eq(&rule_entry)
                        {
                            if (j < i) && !reset_this_loop
                            {
                                // reposition...
                                let item = updates.remove(i);
                                updates.insert(0, item);
                                println!("update reorder: val{}, pos{} to {} - {:?}", item, i, 0, updates);
                                //reset the scan..
                                reset_this_loop = true;
                                break;
                            }
                        }
                        j += 1;
                    }
                    if !reset_this_loop
                    {
                        i +=1;
                    } else {
                        i = 0;
                        was_incorrectly_ordered = true;
                        break;
                    }
                }
            }
            if was_incorrectly_ordered{
                //now correct determine the middle page number
                result += updates[updates.len()/2];
            }
        }
    }
    Some(result)
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
