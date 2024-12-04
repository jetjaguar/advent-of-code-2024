advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result:u32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let mut report:Vec<i32> = Vec::new();
        
        let mut words = line.split(' ');
        while let Some(entry) = words.next()
        {
            match entry.parse::<i32>() {
                Ok(v) => report.push(v),
                Err(_) => {}
            };   
        }

        // tired man flips the logic on the if and is confused when program spits out 4 wrong numbers
        // then tired man doesn't have the logic about no dupes and going up and down right at all

        //let mut increasing:Vec<u32> = report.clone();
        //increasing.sort();
        //let mut decreasing:Vec<u32> = report.clone();
        //decreasing.sort_by(|a, b| b.cmp(a));

        //println!("report {:?}", report);
        //println!("inc {:?}", increasing);
        //println!("{}", report.eq(&increasing));
        //println!("dec {:?}", decreasing);
        //println!("{}", report.eq(&decreasing));

        //if report.eq(&increasing) || report.eq(&decreasing) 
        //{
        //    result += 1;
        //}

        //Shameful debug prints
        println!("report {:?}", report);

        let mut i = 0;
        let mut prev = &report[0];
        let mut incr = false;
        let mut decr = false;
        let mut intermed_result = 1;
        for current in report.iter()
        {
            // determine if incr or decr
            if i == 1
            {
                // I messed up the logic here that led to the debug prints
                incr = prev < current; 
                decr = current < prev;
                println!("inc {}", incr);
                println!("dec {}", decr);
                if !incr && !decr
                {
                    intermed_result = 0;
                    break;
                }
            }
            
            if i > 0 
            {
                let diff = prev - current;
                if  (prev.abs_diff(*current) > 3) || (prev.abs_diff(*current) == 0) || (incr && diff > 0) || (decr && diff < 0)
                {
                    intermed_result = 0;
                    break;
                }
            }
            
            // if 0 or always
            i += 1;
            prev = current;
        }
        println!("int {}", intermed_result);

        result +=intermed_result;
    }

    Some(result)
}

// we can't account for the first position being bad, so we run the function twice and pass in one_bad, I am sad about it
pub fn part_two_a(input: Vec<i32>, one_bad: bool) -> u32 {
    let mut i = 0;
    let mut prev = &input[0];
    let mut incr = false;
    let mut decr = false;
    let mut orientation_found = false;
    let mut one_bad_int = one_bad;
    let mut intermed_result: u32 = 1;
    
    while i < input.len() 
    {
        let current = &input[i];
        // determine if incr or decr
        if !orientation_found && (i > 0)
        {
            incr = prev < current; 
            decr = current < prev;
            if !incr && !decr && one_bad_int
            {
                intermed_result = 0;
                break;
            // Case: We start with a duplicate
            } if !incr && !decr && !one_bad_int {
                one_bad_int = true;
            } else {
                orientation_found = true;
            }
        } else {
            let diff = prev - current;
            if  (prev.abs_diff(*current) > 3) || (prev.abs_diff(*current) == 0) || (incr && diff > 0) || (decr && diff < 0)
            {
                if one_bad_int
                {
                    intermed_result = 0;
                    break;
                } else {
                    one_bad_int = true;
                    
                }
            }
        }
        
        // always
        if one_bad
        {
            i = 0;
            orientation_found = false;
            input.remove(i);
        }
        else
        {
            i += 1;
            prev = current;
        }
        
    }

    return intermed_result;

}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result:u32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let mut report:Vec<i32> = Vec::new();
        
        let mut words = line.split(' ');
        while let Some(entry) = words.next()
        {
            match entry.parse::<i32>() {
                Ok(v) => report.push(v),
                Err(_) => {}
            };   
        }

        let mut intermed_result = part_two_a(report.clone(), false);
        if intermed_result == 0 
        {
            report.remove(0);
            intermed_result = part_two_a(report.clone(), true);
        }

        result +=intermed_result;
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
