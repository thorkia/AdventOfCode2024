use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn part1(lines: &[String]) {
    let mut valid_reports = 0;

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let part1 = parts[0].parse::<i32>().unwrap();
        let part2 = parts[1].parse::<i32>().unwrap();

        let increasing = part2 > part1;
        let mut is_valid = false;
        for i in 1..parts.len() {
            let part = parts[i].parse::<i32>().unwrap();
            let prev_part = parts[i - 1].parse::<i32>().unwrap();
            
            //Check if the current part is following the same direction
            if increasing && part > prev_part {
                is_valid = true;
            } else if !increasing && part < prev_part {
                is_valid = true;
            } else {
                is_valid = false;
                break;
            }

            //check if the difference between the parts is between 1 and 3 if, not, it's not valid
            //we actually don't need to check for <1 as that will be covered by the same check
            if (part - prev_part).abs() > 3 {
                is_valid = false;
                break;
            }
        }

        if is_valid {
            valid_reports += 1;
        }
        
    }

    println!("{}", valid_reports);
}

fn is_valid_pair(part: i32, next_part: i32, increasing: bool) -> bool {
    //If the parts are the same, then it's not valid
    if part == next_part {
        return false;
    }

    //If the parts are more than 3 apart, then it's not valid
    if (part - next_part).abs() > 3 {
        return false;  
    }
    
    //If it's increasing, then the current part must be less than the next part
    if increasing && part >= next_part {
        return false;
    }

    //If it's decreasing, then the current part must be greate than the next part
    if !increasing && part <= next_part {
        return false;
    }

    //If we've made it this far, then it's valid
    return true;
}

fn part2(lines: &[String]) {
    let mut valid_reports = 0;

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let part1 = parts[0].parse::<i32>().unwrap();
        let part2 = parts[1].parse::<i32>().unwrap();

        let mut increasing = part2 > part1;
        let mut dampener = false;
        let mut is_valid = false;

        //Redo this entirely
        let mut i = 0;
        while i+1 < parts.len() {
            let part = parts[i].parse::<i32>().unwrap();
            let next_part = parts[i + 1].parse::<i32>().unwrap();

            if part == next_part {
                if dampener {
                    is_valid = false;
                    break;
                } else {
                    dampener = true;
                    is_valid = true;
                    if i==0 { //if on the first index reset the direction
                        increasing = parts[i+2].parse::<i32>().unwrap() > next_part;
                    }
                }
            }

            if (part - next_part).abs() > 3 {                
                //If the dampener is alreadset, then it's not valid
                if dampener {
                    is_valid = false;
                    break;
                }
                
                //If its the last item and we're not dampened, then it's valid
                if i+1 == parts.len() - 1  && !dampener{
                    is_valid = true;
                    break;
                }            

                dampener = true;
                is_valid = false;

                //If it's the first item, we turn on the dampener
                //We should also recheck the direction.
                if i == 0 {                    
                    increasing = parts[i+2].parse::<i32>().unwrap() > next_part;
                    is_valid = true;
                }
            }

            //Check if increasind and not, then check if decreasing and not
            if increasing && next_part > part {
                is_valid = true;
            } else if !increasing && part < prev_part {
                is_valid = true;
            }

            i += 1;
        }

        /* for i in 1..parts.len() {
            let part = parts[i].parse::<i32>().unwrap();
            let prev_part = parts[i - 1].parse::<i32>().unwrap();
            
            //Check if the current part is following the same direction
            if increasing && part > prev_part {
                is_valid = true;
            } else if !increasing && part < prev_part {
                is_valid = true;
            } else {
                if dampener {
                    is_valid = false;
                    break;
                } else {
                    dampener = true;
                    is_valid = true;
                }
            }

            //if the difference is >3, then skipping an item will just make it bigger, so we can ignore the dampener
            //we don't need to check for less than 1, as that is the same.
            if (part - prev_part).abs() > 3 {
                if i+1 == parts.len() {
                    if dampener {
                        is_valid = false;
                        break;                        
                    } else {
                        is_valid = true;
                    }
                } else {
                    is_valid = false;
                    break;
                }
            }            
        } */

        //println!("{} - {}", line, is_valid);
        if is_valid {
            valid_reports += 1;
            println!("{} - {}", line, is_valid);
            //if dampener {
            //    println!("{} - {}", line, is_valid);
            //}
        } /* else {
            println!("{} - {}", line, is_valid);
        }      */
    }

    println!("{}", valid_reports);
}



fn main() {
    let total_start = Instant::now();
    let Ok(lines) = lines_from_file("./src/test_input.txt") else {
        panic!("Could not read file");
    };

    let mut processed_lines: Vec<String> = Vec::new();
    for line in lines {
        if let Ok(ip) = line {
            processed_lines.push(ip);
        }
    }

    let start = Instant::now();
    part1(&processed_lines);
    let duration = start.elapsed();
    println!("Part 1 took {}microseconds", duration.as_micros());

    let start = Instant::now();
    part2(&processed_lines);
    let duration = start.elapsed();
    println!("Part 2 took {}microseconds", duration.as_micros());



    let duration = total_start.elapsed();
    println!("Total took {}microseconds", duration.as_micros());
}
