pub struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        //6:12
        let mut list_a: Vec<i32> = Vec::new();

        let mut node = &head;
        while node.is_some() {
            list_a.push(node.as_ref().unwrap().val);
            node = &(node.as_ref().unwrap().next);
        }

        let list_b: Vec<i32> = list_a.iter().copied().rev().collect();

        for (a, b) in list_a.iter().zip(list_b.iter()) {
            if a != b {
                return false;
            }
        }

        return true;
        //6:28
    }
}

fn main() {}
