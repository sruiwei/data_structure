/// # [203. 移除链表元素](https://leetcode.cn/problems/remove-linked-list-elements/description/)
/// 
/// 给你一个链表的头节点 head 和一个整数 val ，请你删除链表中所有满足 Node.val == val 的节点，并返回 新的头节点 。
/// 
/// ## 示例 1
/// 
/// ```text
/// 输入：head = [1,2,6,3,4,5,6], val = 6
/// 输出：[1,2,3,4,5]
/// ```
/// 
/// ## 示例 2
/// 
/// ```text
/// 输入：head = [], val = 1
/// 输出：[]
/// ```
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

pub struct Solution;

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut new_head = Box::new(ListNode { val: 0, next: head });

        let mut current_node = new_head.as_mut();

        while let Some(x) = current_node.next.take() {
            if x.val == val {
                current_node.next = x.next;
            } else {
                current_node.next = Some(x);
                current_node = current_node.next.as_mut().unwrap();
            }
        }

        new_head.next
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    #[test]
    fn test_remove_elements() {
        let node6 = Some(Box::new(ListNode { val: 6, next: None }));
        let node5 = Some(Box::new(ListNode {
            val: 5,
            next: node6,
        }));
        let node4 = Some(Box::new(ListNode {
            val: 4,
            next: node5,
        }));
        let node3 = Some(Box::new(ListNode {
            val: 3,
            next: node4,
        }));
        let node6a = Some(Box::new(ListNode {
            val: 6,
            next: node3,
        }));
        let node2 = Some(Box::new(ListNode {
            val: 2,
            next: node6a,
        }));
        let head = Some(Box::new(super::ListNode {
            val: 1,
            next: node2,
        }));
        let res1 = Solution::remove_elements(head, 6);
        assert_eq!(1, res1.unwrap().val);
    }
}
