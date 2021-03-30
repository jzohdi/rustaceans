// Given s list of integeres, use a vector and return the
// mean (the average value)
// median (when sorted the value in the middle pos) and
// mode (the value taht occurs the most)
use std::io;
use std::collections::HashMap;

pub fn analyze_nums() {
    let mut nums: String = String::new();

    loop {
        println!("Please enter a list of integers to analyze separated by spaces.");

        io::stdin()
            .read_line(&mut nums)
            .expect("Failed to read line");

        if is_valid_nums(&nums[..]) {
            break;
        }
        nums = String::new();
    }
    let nums: Vec<i32> = nums
        .split(" ")
        .map(|s| 
            if let Ok(num) = 
                s.trim()
                 .parse::<i32>() { num } else { 0 })
        .collect();
    println!("mean: {}\nmedian: {}\nmode: {}",
                mean(&nums), median(&nums), mode(&nums));
}

fn mean(nums: &Vec<i32>) -> f32 {
    let total = nums.iter().fold(0, |mut sum, &x| { sum += x; sum });
    return total as f32 / nums.len() as f32;
}

fn median(nums: &Vec<i32>) -> f32 {
    let mut nums_copy = nums.clone();
    nums_copy.sort();
    let mid = nums_copy.len() / 2 as usize;
    if nums_copy.len() % 2 == 0 {
        let first = nums_copy[mid - 1];
        let second = nums_copy[mid];
        return ((first as f32) + (second as f32))/2.0;
    }
    nums_copy[mid] as f32
}

fn mode(nums: &Vec<i32>) -> f32 {
    let mut most_seen: Option<i32> = None;
    let mut seen_count = 0;
    let mut map = HashMap::new();

    for item in nums.iter() {
        let count = map.entry(item).or_insert(0);
        *count += 1;
        if *count > seen_count {
            seen_count = *count;
            most_seen = Some(*item);
        }
    }
    if let Some(num) = most_seen { num as f32 } else { 0.0 } 
}


fn is_valid_nums(num_str: &str) -> bool {
   for word in num_str.trim().split_whitespace() {
        if let Err(_) = word.trim().parse::<i32>() {
            return false;
        }
   }
   true
}
