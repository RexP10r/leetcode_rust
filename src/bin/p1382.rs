use std::cell::RefCell;
use std::rc::Rc;
use leetcode_rust::ds::leetcode_bst::TreeNode;

struct Solution;

impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let sorted_vec: Vec<i32>;
        if let Some(ref node) = root {
            sorted_vec = node.borrow().inorder();
        } else {
            return None;
        }
        TreeNode::constract_balaced(sorted_vec.as_slice())
    }
}

fn main() {
    let _tmp = Solution::balance_bst(None);
}
