/*
 * @lc app=leetcode id=1221 lang=rust
 *
 * [1221] Split a String in Balanced Strings
 */

// @lc code=start
impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut count = 0;
        let mut l_count: u16 = 0;
        let mut r_count = 0;
        for ch in s.chars() {
            if ch == 'R' {
                r_count += 1;
            }
            if ch == 'L' {
                l_count += 1;
            }
            if r_count == l_count {
                count += 1;
                r_count = 0;
                l_count = 0;
            }
        }
        count
    }
}
// @lc code=end

