/**
 * [05] Longest Palindromic Substring
 * 
 * Given a string s, return the longest palindromic substring in s.
 * 
 * 
 * Example 1:
 * 
 * Input: s = "babad"
 * Output: "bab"
 * Explanation: "aba" is also a valid answer.
 * 
 * Example 2:
 * Input: s = "cbbd"
 * Output: "bb"
*/

pub struct Solution {}

impl Solution {
  pub fn longest_palindrome(s: String) -> String {
    if s.len() < 1 {
        return s
    }
    let chars: Vec<char> = s.chars().collect();
    let mut result: String = String::from("");
    let len: usize = chars.len();
    if len == 2 && chars[0] == chars[1] {
        return s
    }
    for index in 1..len-1 {
        if let (Some(prev_character), Some(_next_character)) = (
            index.checked_sub(1).and_then(|i| chars.get(i)),
            chars.get(index+1)
        ) {
            let character: char = chars[index];
            let mut word: String = String::from("");
            let mut prev_index = index - 1;
            let mut next_index = index + 1;
            if character == chars[next_index] {
                if index == 1 && *prev_character == character {
                    word.push(*prev_character);
                }
                word.push(character);
                while next_index < len && character == chars[next_index] {
                    word.push(chars[next_index]);
                    next_index += 1;
                }
                if word.len() % 2 == 0 {
                    while next_index < len && chars[prev_index] == chars[next_index] {
                        word = s[prev_index..=next_index].to_string();
                        prev_index = prev_index.saturating_sub(1);
                        next_index += 1;
                    }
                } 
                if word.len() > result.len() {
                    result = word.clone();
                }
            } else {
                if index == 1 && *prev_character == character {
                    result = s[prev_index..=index].to_string();
                }
            }
            word.clear();
            prev_index = index - 1;
            next_index = index + 1;
            if chars[prev_index] == chars[next_index] {
                while next_index < len && chars[prev_index] == chars[next_index] {
                    word = s[prev_index..=next_index].to_string();
                    if prev_index > 0 {
                        prev_index = prev_index.saturating_sub(1);
                        next_index += 1;
                    } else {
                        break;
                    }
                }
                if word.len() > result.len() {
                    result = word;
                }
            }
        }  
    }
    if result.len() == 0 {
        return s.chars().take(1).collect()
    }
    result
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test_05() {
    assert_eq!(Solution::longest_palindrome(String::from("babad")), "bab");
    assert_eq!(Solution::longest_palindrome(String::from("cbbd")), "bb");
    assert_eq!(Solution::longest_palindrome(String::from("ac")), "a");
    assert_eq!(Solution::longest_palindrome(String::from("ccc")), "ccc");
    assert_eq!(Solution::longest_palindrome(String::from("abcda")), "a");
    assert_eq!(Solution::longest_palindrome(String::from("aacabdkacaa")), "aca");
    assert_eq!(Solution::longest_palindrome(String::from("xaabacxcabaaxcabaax")), "xaabacxcabaax");
    assert_eq!(Solution::longest_palindrome(String::from("babadada")), "adada");
    assert_eq!(Solution::longest_palindrome(String::from("aaaabaaa")), "aaabaaa");
  }
}