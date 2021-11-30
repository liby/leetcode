/*
 * @lc app=leetcode id=501 lang=rust
 *
 * [501] Find Mode in Binary Search Tree
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
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => vec![],
            Some(root) => {
                let mut counter = std::collections::HashMap::<i32, i32>::new();
                fn find(
                    n: Option<Rc<RefCell<TreeNode>>>,
                    m: &mut std::collections::HashMap<i32, i32>,
                ) {
                    match n {
                        Some(n) => {
                            (*m.entry(n.borrow().val).or_insert(0)) += 1;
                            find(n.borrow().left.clone(), m);
                            find(n.borrow().right.clone(), m);
                        }
                        None => (),
                    }
                }
                find(Some(root), &mut counter);
                let mut list = counter.iter().collect::<Vec<(&i32, &i32)>>();
                list.sort_unstable_by(|a, b| b.1.cmp(a.1));

                let (mut res, mut prev) = (vec![*list[0].0], list[0].1);
                for i in 1..list.len() {
                    if prev != list[i].1 {
                        break;
                    }
                    res.push(*(list[i].0));
                }
                res
            }
        }
    }
}
// @lc code=end
