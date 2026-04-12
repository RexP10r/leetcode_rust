pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
impl TreeNode {
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

struct Solution;

impl TreeNode {
    pub fn inorder(&self) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        self.inorder_rec(&mut result);
        result
    }
    fn inorder_rec(&self, result: &mut Vec<i32>) {
        if let Some(ref left) = self.left {
            left.borrow().inorder_rec(result);
        }
        result.push(self.val);
        if let Some(ref right) = self.right {
            right.borrow().inorder_rec(result);
        }
    }
    pub fn constract_balaced(source: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if source.is_empty() {
            return None;
        };
        let mid = source.len() / 2;
        let mut root = Self::new(source[mid]);
        root.left = Self::constract_balaced(&source[..mid]);
        root.right = Self::constract_balaced(&source[mid + 1..]);
        Some(Rc::new(RefCell::new(root)))
    }
}
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
