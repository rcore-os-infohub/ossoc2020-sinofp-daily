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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1.as_ref();
        let mut l2 = l2.as_ref();
        let mut head = Box::new(ListNode::new(0));
        let mut cur = &mut head;
        let mut carry = 0i32;
        while l1.is_some() || l2.is_some() || carry != 0 {
            // 对Option使用map直接处理Some可真是方便
            let sum = l1.map_or(0, |l| l.val) + l2.map_or(0, |l| l.val) + carry;
            carry = sum / 10;
            cur.next = Some(Box::new(ListNode::new(sum % 10)));
            cur = cur.next.as_mut().unwrap();
            l1 = l1.map(|l| l.next.as_ref()).unwrap_or(None);
            l2 = l2.map(|l| l.next.as_ref()).unwrap_or(None);
        }
        head.next
    }
}

struct Solution;

fn main() {
    // 比较麻烦，直接用OJ测试了
    println!("ok")
}
