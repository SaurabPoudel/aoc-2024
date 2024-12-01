use std::{
    fs::{read_to_string},
};


fn read_pairs(filename: &str) -> Result<Vec<(i32, i32)>, Box<dyn std::error::Error>> {
    let content = read_to_string(filename)?;
    
    content
        .lines()
        .enumerate()
        .map(|(line_num, line)| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|x| {
                    x.parse().map_err(|_| {
                        format!("Failed to parse number on line {}", line_num + 1)
                    })
                })
                .collect::<Result<Vec<i32>, String>>()?;
            
            if nums.len() != 2 {
                return Err(format!("Line {} must contain exactly 2 numbers", line_num + 1).into());
            }
            
            Ok((nums[0], nums[1]))
        })
        .collect()
}

fn sort_pairs(pairs: Vec<(i32, i32)>) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = pairs.iter().map(|(x, _)| *x).collect();
    let mut right: Vec<i32> = pairs.iter().map(|(_, y)| *y).collect();
    
    left.sort_unstable();
    right.sort_unstable();
    
    (left, right)
}

fn calculate_distance(left: &[i32], right: &[i32]) -> i32 {
    left.iter()
        .zip(right.iter())
        .map(|(x, y)| (x - y).abs())
        .sum()
}

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let pairs = read_pairs("input/day1.txt")?;
    let (left_sorted, right_sorted) = sort_pairs(pairs);
    let total_distance = calculate_distance(&left_sorted, &right_sorted);
    println!("Total distance: {}", total_distance);
    Ok(())
}