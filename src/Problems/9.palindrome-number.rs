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
        let mut digits: [u8; 11] = [0; 11];
        let mut length = 0;
        let mut temp = x.clone();
        while temp > 0 {
            digits[length] = (temp % 10) as u8;
            temp /= 10;
            length += 1;
        }
        for i in 0..length / 2 {
            if digits[i] != digits[length - 1 - i] {
                return false
            }
        }
        true
    }
}
// @lc code=end

