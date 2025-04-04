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
        let mut digits = 0;
        let mut temp = x.clone();
        while temp > 0 {
            digits += 1;
            temp /= 10;
        }
        println!("Digits = {}", digits);
        for right_place in 1..=digits as u32 {
            let left_place = digits - right_place + 1;
            let right_digit = x.clone() as i64 % 10_i64.pow(right_place) / 10_i64.pow(right_place - 1);
            let left_digit = x.clone() as i64 % 10_i64.pow(left_place) / 10_i64.pow(left_place - 1);
            if right_digit != left_digit {
                return false
            }
        }
        true
    }
}
// @lc code=end

