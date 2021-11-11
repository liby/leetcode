/*
 * @lc app=leetcode id=412 lang=rust
 *
 * [412] Fizz Buzz
 */

// @lc code=start
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .map(|x| {
                let by3 = x % 3 == 0;
                let by5 = x % 5 == 0;
                match (by3, by5) {
                    (true, true) => "FizzBuzz".into(),
                    (true, false) => "Fizz".into(),
                    (false, true) => "Buzz".into(),
                    (false, false) => x.to_string(),
                }
            })
            .collect()
    }
}
// @lc code=end
