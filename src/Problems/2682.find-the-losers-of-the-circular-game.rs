/*
 * @lc app=leetcode id=2682 lang=rust
 *
 * [2682] Find the Losers of the Circular Game
 */

// @lc code=start
impl Solution {
    pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
        let mut ball_passed = Vec::<i32>::new();
        let mut ball = 0;
        let mut step = k;
        loop {
            if ball_passed.contains(&(ball + 1)) {
                break;
            }
            ball_passed.push(ball + 1);
            ball = (ball + step) % n;
            step += k;
        }

        (1..=n).filter(|x| !ball_passed.contains(&x)).collect::<Vec<i32>>()
    }
}
// @lc code=end

