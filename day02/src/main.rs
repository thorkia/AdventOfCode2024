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

fn part2(lines: &[String]) {
    let mut valid_reports = 0;

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let part1 = parts[0].parse::<i32>().unwrap();
        let part2 = parts[1].parse::<i32>().unwrap();

        let mut increasing = part2 > part1;
        let mut dampener = false;
        let mut is_valid = false;

        for i in 1..parts.len() {
            let part = parts[i].parse::<i32>().unwrap();
            let prev_part = parts[i - 1].parse::<i32>().unwrap();
            
            if i==1 && part == prev_part {
                dampener = true;
                increasing = part < parts[2].parse::<i32>().unwrap();
                continue;
            }

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
                    //check if it is location 1?
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
                } else if i==1 {
                    dampener = true;
                    increasing = part < parts[2].parse::<i32>().unwrap(); //reset the direction
                } else {
                    is_valid = false;
                    break;
                }
            }
        }

        println!("{} - {} - {}", line, is_valid, dampener);
        if is_valid {
            valid_reports += 1;            
        } /* else {
            println!("{} - {}", line, is_valid);
        }      */
    }

    println!("{}", valid_reports);
}



fn main() {
    let total_start = Instant::now();
    let Ok(lines) = lines_from_file("./src/input.txt") else {
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
