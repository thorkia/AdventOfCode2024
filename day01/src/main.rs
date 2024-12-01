
use std::collections::HashMap;
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
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let part1 = parts[0].parse::<i32>().unwrap();
        let part2 = parts[1].parse::<i32>().unwrap();
        
        col1.push(part1);
        col2.push(part2);
    
    }

    //Sort the rows
    col1.sort();
    col2.sort();

    //Iterate over the rows
    let mut row_diff = 0;
    for i in 0..col1.len() {
        row_diff += (col1[i] - col2[i]).abs();
    }

    println!("{}", row_diff);
}

fn part2(lines: &[String]) {
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: HashMap<i32, i32> = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let part1 = parts[0].parse::<i32>().unwrap();
        let part2 = parts[1].parse::<i32>().unwrap();
        
        col1.push(part1);
        if col2.contains_key(&part2) {
            col2.insert(part2, col2.get(&part2).unwrap() + 1);
        } else {
            col2.insert(part2, 1);
        }
    }

    //Iterate over the rows
    let mut total: i64 = 0;
    for i in 0..col1.len() {
        if col2.contains_key(&col1[i]) {
            let count = col2.get(&col1[i]).unwrap();
            total += i64::from( (col1[i] * count).abs() );
        }
    }

    println!("{}", total);

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
