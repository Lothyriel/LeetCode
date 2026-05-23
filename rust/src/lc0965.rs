use std::cell::RefCell;
use std::rc::Rc;

pub fn is_unival_tree(root: Option<Node>) -> bool {
    let Some(root) = root else {
        return true;
    };

    let val = root.borrow().val;

    dfs(Some(&root), val)
}

fn dfs(node: Option<&Node>, expected: i32) -> bool {
    let Some(node) = node else {
        return true;
    };

    let node = node.borrow();

    if node.val != expected {
        return false;
    }

    dfs(node.right.as_ref(), expected) && dfs(node.left.as_ref(), expected)
}

type Node = Rc<RefCell<TreeNode>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Node>,
    pub right: Option<Node>,
}

impl TreeNode {
    pub fn new(val: i32) -> Option<Node> {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        })))
    }
}

impl From<TreeNode> for Option<Node> {
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
