#![allow(dead_code)]
fn main () { println!("Leetcode is fun!"); }

// Given an array of integers nums, half of the integers in nums are odd, and the other half are even.
// 
// Sort the array so that whenever nums[i] is odd, i is odd, and whenever nums[i] is even, i is even.
// 
// Return any answer array that satisfies this condition.
// 
struct Solution {}

impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut values = nums.clone();
        let mut sorted: Vec<i32> = Vec::new();

        for idx in 0..nums.len() {
            let i: usize;

            if idx % 2 == 0 {
                i = values.iter().position(|x| x % 2 == 0).unwrap();
            } else {
                i = values.iter().position(|x| x % 2 != 0).unwrap();    
            }

            let v = values.remove(i);
            sorted.push(v);
        }
           
        return sorted;
    }
}


#[test]
fn should_return_an_array_in_right_sorted_order_test_1() {
    let nums = vec![4,2,5,7];
    let res = Solution::sort_array_by_parity_ii(nums);
    
    assert_eq!(res, vec![4,5,2,7]);
}

#[test]
fn should_return_an_array_in_right_sorted_order_test_2() {
    let nums = vec![2,3];
    let res = Solution::sort_array_by_parity_ii(nums);
    
    assert_eq!(res, vec![2,3]);
}
