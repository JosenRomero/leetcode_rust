/**
 * [01] Two Sum
 * 
 * Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
 * 
 * You may assume that each input would have exactly one solution, and you may not use the same element twice.
 * 
 * You can return the answer in any order.
 * 
 * 
 * Example 1:
 * 
 * Input: nums = [2,7,11,15], target = 9
 * Output: [0,1]
 * Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
*/

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut map: HashMap<i32, i32> = HashMap::new(); // <num, num_index>
    let mut i = 0;
    
    while result.len() < 1 && i < nums.len() {
        
      let complement: i32 = target - nums[i];
  
      if let Some(&index) = map.get(&complement) {
        result.push(index); // the index of the complement number 
        result.push(i as i32); // the index of the current number
      }
          
      map.insert(nums[i] as i32, i as i32);
      
      i += 1;
        
    }
    
    result
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test_01() {
    assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), [0,1]);
    assert_eq!(Solution::two_sum(vec![3,2,4], 6), [1,2]);
    assert_eq!(Solution::two_sum(vec![3,3], 6), [0,1]);
  }
}