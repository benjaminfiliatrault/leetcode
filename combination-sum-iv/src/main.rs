#![allow(dead_code)]
use std::collections::HashMap;

fn main() { println!("Hello, world!");  Solution::combination_sum4(vec![1,2,3], 5);}

// Given an array of distinct integers nums and a target integer target,
// return the number of possible combinations that add up to target.

struct Solution {}

impl Solution {
    
    fn dynamic_prog(nums: Vec<i32>, target: i32) -> i32 {
        let mut results = vec![0; (target + 1) as usize];
        results[0] = 1;

        for i in 1..=target as usize {
            for &num in &nums {
                if i as i32 - num >= 0 {
                    results[i] += results[(i as i32 - num) as usize];
                }
            }
        }

        results[target as usize]
    }

    fn recursion_memo(nums: &Vec<i32>, target: i128, memo: &mut HashMap<i128, i128>) -> i128 {
        if let Some(&count) = memo.get(&target) { return count; }

        if target == 0 { return 1 };

        if target < i128::from(nums[0]) { return 0 };

        let mut count: i128 = 0;

        for &num in nums {
            let sum = target - i128::from(num);
            if sum < 0 { break };
            count += Self::recursion_memo(nums, sum, memo);
            println!("{count}");
        }

        memo.insert(target, count);

        count 
    }

    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i128 {
       // return Self::dynamic_prog(nums, target)
       
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        let mut memo = HashMap::new();
        
        Self::recursion_memo(&nums, target.into(), &mut memo)
    }
}

#[test]
fn should_return_7() {
    let nums = vec![1,2,3];
    let target = 4;

    let result = Solution::combination_sum4(nums, target);

    assert_eq!(result, 7);
}

#[test]
fn should_return_0() {
    let nums = vec![9];
    let target = 3;

    let result = Solution::combination_sum4(nums, target);

    assert_eq!(result, 0);
}

#[test]
fn should_return_1() {
    let nums = vec![3];
    let target = 3;

    let result = Solution::combination_sum4(nums, target);

    assert_eq!(result, 1);
}

#[test]
fn should_return() {
    let nums = vec![1,2,3,4,5,6,7,9,10,11,12,13];
    let target = 100;

    let result = Solution::combination_sum4(nums, target);

    assert_eq!(result, 1);
}
