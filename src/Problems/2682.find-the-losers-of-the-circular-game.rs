/*
 * @lc app=leetcode id=2682 lang=rust
 *
 * [2682] Find the Losers of the Circular Game
 */

// @lc code=start
impl Solution {
    pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
        let mut ball_passed = vec![false; n as usize];
        let mut ball: i32 = 0;
        let mut step = k;

        loop {
            if ball_passed[ball as usize] {
                break
            }
            ball_passed[ball as usize] = true;
            ball = (ball + step) % n;
            step += k
        }

        let mut losers = Vec::<i32>::new();
        for (i, passed) in ball_passed.iter().enumerate() {
            if !*passed {
                losers.push((i + 1) as i32)
            }
        }

        losers


    }
}
// @lc code=end

