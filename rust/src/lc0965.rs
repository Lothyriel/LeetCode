use std::cell::RefCell;
use std::rc::Rc;

pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(root) = root {
        let root = root.borrow();

        return dfs(root.right.clone(), root.val) && dfs(root.left.clone(), root.val);
    }

    true
}

fn dfs(node: Option<Rc<RefCell<TreeNode>>>, num: i32) -> bool {
    if let Some(root) = node {
        let node = root.borrow();

        if node.val != num {
            return false;
        }

        return dfs(node.right.clone(), num) && dfs(node.left.clone(), num);
    }

    true
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        })))
    }
}

impl From<TreeNode> for Option<Rc<RefCell<TreeNode>>> {
    fn from(value: TreeNode) -> Self {
        Some(Rc::new(RefCell::new(value)))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let tree = TreeNode {
            val: 1,
            left: TreeNode {
                val: 1,
                left: TreeNode::new(1),
                right: TreeNode::new(1),
            }
            .into(),
            right: TreeNode {
                val: 1,
                right: TreeNode::new(1),
                left: None,
            }
            .into(),
        };

        assert!(is_unival_tree(tree.into()));

        let tree = TreeNode {
            val: 2,
            left: TreeNode {
                val: 2,
                left: TreeNode::new(5),
                right: TreeNode::new(2),
            }
            .into(),
            right: TreeNode::new(2),
        };

        assert!(!is_unival_tree(tree.into()));
    }
}
