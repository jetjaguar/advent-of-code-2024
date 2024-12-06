advent_of_code::solution!(4);

#[derive(Debug)]
struct Tuple
{
    x_coord: i32,
    y_coord: i32,
}

pub fn generate_tuples (x_loc: i32 , y_loc: i32 , list_height: usize , list_width: usize) -> Vec<Tuple>
{
    let mut return_vec: Vec<Tuple> = Vec::new();
    
    // Top row
    if (x_loc - 1) >= 0 
    {
        //  Numpad 7
        if (y_loc - 1) >= 0 
        {
            return_vec.push(Tuple{
                x_coord: x_loc - 1,
                y_coord: y_loc - 1
            });
        }
        // 8
        return_vec.push(Tuple{
            x_coord: x_loc - 1,
            y_coord: y_loc
        });

        // 9
        if (y_loc + 1) < list_width.try_into().unwrap()
        {
            return_vec.push(Tuple{
                x_coord: x_loc - 1,
                y_coord: y_loc + 1
            });    
        }
    }

    // Same Row 4
    if (y_loc - 1) >= 0 
    {
        return_vec.push(Tuple{
            x_coord: x_loc,
            y_coord: y_loc - 1
        });
    }

    // 6
    if (y_loc + 1) < list_width.try_into().unwrap()
    {
        return_vec.push(Tuple{
            x_coord: x_loc,
            y_coord: y_loc + 1
        });    
    }

    // Next Row 1
    if (x_loc + 1) < list_height.try_into().unwrap()
    {
        if (y_loc - 1) >= 0 
        {
            return_vec.push(Tuple{
                x_coord: x_loc + 1,
                y_coord: y_loc - 1
            });    
        }     
        // 2
        return_vec.push(Tuple{
            x_coord: x_loc + 1,
            y_coord: y_loc
        });

        // 3
        if (y_loc + 1) < list_width.try_into().unwrap()
        {
            return_vec.push(Tuple{
                x_coord: x_loc + 1,
                y_coord: y_loc + 1
            });    
        }
    }

    return return_vec;
}

pub fn part_one(input: &str) -> Option<u32> {
    //Create the 2d vector from the input
    let mut my_list: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        my_list.push(line.to_string().chars().collect());
    }

    let list_height = my_list.len();
    let list_width = my_list[0].len();

    let mut result = 0;
    let mut x_cursor = 0;
    //Scan the vector, switch on which letter you find
    while x_cursor < list_height 
    {
        //println!("row {:?}", my_list[x_cursor]);
        let mut y_cursor = 0;

        while y_cursor < list_width
        {
            //If X, then an M should be adjacent
            if my_list[x_cursor][y_cursor].eq(&'X')
            {
                for pair_m in generate_tuples(x_cursor as i32, y_cursor as i32, list_height, list_width)
                {
                    //If M, then an A should be adjacent
                    if my_list[pair_m.x_coord as usize][pair_m.y_coord as usize].eq(&'M')
                    {
                        //print!(" M/{},{}/{}", pair_m.x_coord, pair_m.y_coord, my_list[pair_m.x_coord as usize][pair_m.y_coord as usize]);
                        for pair_a in generate_tuples(pair_m.x_coord, pair_m.y_coord, list_height, list_width)
                        {        
                            //If A, then an S should be adjacent
                            if my_list[pair_a.x_coord as usize][pair_a.y_coord as usize].eq(&'A')
                            {
                                //print!(" A/{},{}/{}", pair_a.x_coord, pair_a.y_coord, my_list[pair_a.x_coord as usize][pair_a.y_coord as usize]);
                                for pair_s in generate_tuples(pair_a.x_coord, pair_a.y_coord, list_height, list_width)
                                {
                                    if my_list[pair_s.x_coord as usize][pair_s.y_coord as usize].eq(&'S')
                                    {
                                        //println!("S/{},{}/{}", pair_s.x_coord, pair_s.y_coord, my_list[pair_s.x_coord as usize][pair_s.y_coord as usize]);
                                        result += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            y_cursor += 1;
        }
        x_cursor += 1;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
