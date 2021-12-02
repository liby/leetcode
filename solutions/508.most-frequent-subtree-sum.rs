/*
 * @lc app=leetcode id=508 lang=rust
 *
 * [508] Most Frequent Subtree Sum
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => vec![],
            Some(root) => {
                let mut m = std::collections::HashMap::new();
                fn count(
                    root: Option<Rc<RefCell<TreeNode>>>,
                    m: &mut std::collections::HashMap<i32, i32>,
                ) -> i32 {
                    match root {
                        Some(root) => {
                            let left = count(root.borrow().left.clone(), m);
                            let right = count(root.borrow().right.clone(), m);
                            let total = left + right + root.borrow().val;
                            (*m.entry(total).or_insert(0)) += 1;
                            total
                        }
                        None => 0,
                    }
                }
                count(Some(root), &mut m);
                let mut list = m.iter().collect::<Vec<(&i32, &i32)>>();
                list.sort_unstable_by(|a, b| b.1.cmp(a.1));

                let (mut res, mut prev) = (vec![*list[0].0], list[0].1);
                for i in 1..list.len() {
                    if prev != list[i].1 {
                        break;
                    }
                    res.push(*list[i].0);
                }
                res
            }
        }
    }
}
// @lc code=end
