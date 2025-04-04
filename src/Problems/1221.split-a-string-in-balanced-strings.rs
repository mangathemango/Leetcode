/*
 * @lc app=leetcode id=1221 lang=rust
 *
 * [1221] Split a String in Balanced Strings
 */

// @lc code=start
impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut count = 0;
        let mut balance: u16 = 0;
        for ch in s.chars() {
            match ch {
                'R' => balance += 1,
                'L' => balance -= 1,
                _ => ()
            }
            if balance == 0 {
                count += 1;
            }
        }
        count
    }
}
// @lc code=end

