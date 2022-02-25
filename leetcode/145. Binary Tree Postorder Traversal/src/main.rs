pub struct Solution {}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn traval(node: Option<Rc<RefCell<TreeNode>>>, result_list: &mut Vec<i32>) {
        if let Some(node) = node {
            let node = node.borrow();
            Solution::traval(node.left.clone(), result_list);
            Solution::traval(node.right.clone(), result_list);
            result_list.push(node.val);
        }
    }
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        //9:11
        let mut result_list = vec![];
        Self::traval(root, &mut result_list);
        return result_list;
        //9:14
    }
}

fn main() {}
