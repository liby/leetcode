/*
 * @lc app=leetcode id=557 lang=rust
 *
 * [557] Reverse Words in a String III
 */

// @lc code=start
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut temp: String = String::from("");
        let sp:Vec<&str> = s.split(' ').collect();

        for (_,v) in sp.iter().enumerate() {
            for k in v.chars().rev(){
                temp.push(k);
            }
            temp.push_str(" ");
        }
        if temp.chars().nth(temp.len()-1) == Some(' '){
            temp.pop();
        }
        return temp;
    }
}
// @lc code=end

