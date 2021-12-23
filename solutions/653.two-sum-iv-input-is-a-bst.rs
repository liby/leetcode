/*
 * @lc app=leetcode id=653 lang=rust
 *
 * [653] Two Sum IV - Input is a BST
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
use std::collections::HashSet;
use std::rc::Rc;
impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut set: HashSet<i32> = HashSet::new();
        return in_order(&root, &mut set, &k);

        fn in_order(root: &Option<Rc<RefCell<TreeNode>>>, set: &mut HashSet<i32>, k: &i32) -> bool {
            if root.is_none() {
                return false;
            }

            let r = root.as_ref().unwrap().borrow();
            if in_order(&r.left, set, k) || set.contains(&(k - r.val)) {
                return true;
            }

            set.insert(r.val);
            in_order(&r.right, set, k)
        }
    }
}
// @lc code=end
