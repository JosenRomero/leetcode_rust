/**
 * [58] Length of Last Word
 * 
 * Given a string s consisting of words and spaces, return the length of the last word in the string.
 * 
 * A word is a maximal consisting of non-space characters only.
 * 
 * 
 * Example 1:
 * 
 * Input: s = "Hello World"
 * Output: 5
 * Explanation: The last word is "World" with length 5.
*/

pub struct Solution {}

impl Solution {
  pub fn length_of_last_word(s: String) -> i32 {
    let words: Vec<&str> = s.as_str().split_whitespace().collect();
    let last_word: &str = words.get(words.len()-1).unwrap();
    return last_word.len() as i32
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test_58() {
    assert_eq!(Solution::length_of_last_word("Hello World".to_owned()), 5); // The last word is "World" with length 5.
    assert_eq!(Solution::length_of_last_word("   fly me   to   the moon  ".to_owned()), 4); // The last word is "moon" with length 4.
    assert_eq!(Solution::length_of_last_word("luffy is still joyboy".to_owned()), 6); // The last word is "joyboy" with length 6.
  }
}