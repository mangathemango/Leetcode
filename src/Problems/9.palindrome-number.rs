/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false
        }
        let mut digits = Vec::<u8>::new();
        let mut temp = x.clone();
        while temp > 0 {
            digits.push((temp % 10) as u8);
            temp /= 10;
        }
        for i in 0..digits.len() {
            if digits[i] != digits[digits.len() - 1 - i] {
                return false
            }
        }
        true
    }
}
// @lc code=end

