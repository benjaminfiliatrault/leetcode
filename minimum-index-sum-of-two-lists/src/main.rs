#![allow(dead_code, unused_macros)]
fn main() { println!("Hello, world!");}


// Given two arrays of strings list1 and list2, find the common strings with the least index sum.
// 
// A common string is a string that appeared in both list1 and list2.
// 
// A common string with the least index sum is a common string such that if it appeared at list1[i] 
// and list2[j] then i + j should be the minimum value among all the other common strings.
// 
// Return all the common strings with the least index sum. Return the answer in any order.

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}


struct Solution {}

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut duplicates = Vec::new();
        let mut lowest_idx: Option<usize> = None;

        for (idx, value) in list1.iter().enumerate() {
            if let Some(position) = list2.iter().position(|x| x == value) { 
                let current_idx = idx + position;

                if lowest_idx.is_none() {
                    lowest_idx = Some(current_idx);
                }

                if current_idx <= lowest_idx.unwrap(){
                    
                    // That means last object wasn't the smallest combined
                    // idx so we need to remove it
                    if current_idx < lowest_idx.unwrap() {
                        duplicates.pop();
                    }


                    duplicates.push(value.to_string());
                    lowest_idx = Some(current_idx);
                }
            }
        }

        return duplicates
    }
}


#[test]
fn should_return_shogun_test_1() {
    // The only common string is "Shogun". 
    let list1: Vec<String> = vec_of_strings!["Shogun","Tapioca Express","Burger King","KFC"];
    let list2: Vec<String> = vec_of_strings!["Piatti","The Grill at Torrey Pines","Hungry Hunter Steakhouse","Shogun"];

    let result = Solution::find_restaurant(list1, list2);

    assert_eq!(result, vec!["Shogun"]);
}

#[test]
fn should_return_shogun_test_2() {
    // The common string with the least index sum is "Shogun" with index sum = (0 + 1) = 1.
    let list1: Vec<String> = vec_of_strings!["Shogun","Tapioca Express","Burger King","KFC"];
    let list2: Vec<String> = vec_of_strings!["KFC","Shogun","Burger King"];

    let result = Solution::find_restaurant(list1, list2);

    assert_eq!(result, vec!["Shogun"]);
}

#[test]
fn should_return_sad_and_happy() {
    // There are three common strings:
    // "happy" with index sum = (0 + 1) = 1.
    // "sad" with index sum = (1 + 0) = 1.
    // "good" with index sum = (2 + 2) = 4.
    // The strings with the least index sum are "sad" and "happy".
    let list1: Vec<String> = vec_of_strings!["happy","sad","good"];
    let list2: Vec<String> = vec_of_strings!["sad","happy","good"];

    let result = Solution::find_restaurant(list1, list2).sort(); // Any order

    assert_eq!(result, vec!["sad","happy"].sort());
}

#[test]
fn should_return_piatti() {
    // There are three common strings:
    // "happy" with index sum = (0 + 1) = 1.
    // "sad" with index sum = (1 + 0) = 1.
    // "good" with index sum = (2 + 2) = 4.
    // The strings with the least index sum are "sad" and "happy".
    let list1: Vec<String> = vec_of_strings!["Shogun","Piatti","Tapioca Express","Burger King","KFC"];
    let list2: Vec<String> = vec_of_strings!["Piatti","The Grill at Torrey Pines","Hungry Hunter Steakhouse","Shogun"];

    let result = Solution::find_restaurant(list1, list2); // Any order

    assert_eq!(result, vec!["Piatti"]);
}
