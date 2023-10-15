use std::collections::HashMap;

fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut num_indices = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&j) = num_indices.get(&complement) {
            return Some((i, j));
        }

        num_indices.insert(num, i);
    }

    None
}

fn main() {
    let nums = vec![1, 10, 2, 7];
    let target = 9;
    
    match two_sum(&nums, target) {
        Some((index1, index2)) => {
            println!("Indices of two numbers that add up to the target: {} and {}", index1, index2);
        },
        None => {
            println!("No such numbers found.");
        }
    }
}
