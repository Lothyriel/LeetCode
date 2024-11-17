#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

type Node = Option<Box<ListNode>>;

pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let finished = |a: &Node, b: &Node| a.is_none() && b.is_none();
    let val = |v: &Node| v.as_ref().map(|v| v.val).unwrap_or(0);
    let next = |n: Node| n.and_then(|n| n.next);

    let mut first: Node = None;
    let mut carry = 0;
    let mut current = &mut first;

    while !finished(&l1, &l2) || carry != 0 {
        let sum = carry + val(&l1) + val(&l2);
        carry = sum / 10;

        let new = current.insert(Box::new(ListNode::new(sum % 10)));
        current = &mut new.next;
        l1 = next(l1);
        l2 = next(l2);
    }

    first
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let l1 = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode::new(3))),
                val: 4,
            })),
            val: 2,
        }));

        let l2 = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode::new(4))),
                val: 6,
            })),
            val: 5,
        }));

        let expected = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode::new(8))),
                val: 0,
            })),
            val: 7,
        }));

        assert_eq!(expected, add_two_numbers(l1, l2));
    }
}
