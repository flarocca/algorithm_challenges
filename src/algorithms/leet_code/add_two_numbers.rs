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

    pub fn to_vec(&self) -> Vec<i32> {
        let mut out = Vec::new();
        let mut current: Option<&ListNode> = Some(self);

        while let Some(node) = current {
            out.push(node.val);
            current = node.next.as_deref();
        }

        out
    }
}

impl From<Vec<i32>> for ListNode {
    fn from(values: Vec<i32>) -> Self {
        assert!(
            !values.is_empty(),
            "Cannot create a ListNode from an empty slice"
        );

        let mut head = Box::new(ListNode::new(values[0]));
        let mut tail = &mut head;

        for &v in &values[1..] {
            tail.next = Some(Box::new(ListNode::new(v)));
            tail = tail.next.as_mut().unwrap();
        }

        *head
    }
}

pub struct Solution;

impl Solution {
    /// You are given two non-empty linked lists representing two non-negative integers.
    /// The digits are stored in reverse order, and each of their nodes contains a single digit.
    /// Add the two numbers and return the sum as a linked list.
    ///
    /// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
    ///
    /// Example 1:
    ///     Input: l1 = [2,4,3], l2 = [5,6,4]
    ///     Output: [7,0,8]
    ///     Explanation: 342 + 465 = 807.
    ///
    /// Example 2:
    ///     Input: l1 = [0], l2 = [0]
    ///     Output: [0]
    ///     Explanation: 0 + 0 = 0.
    ///
    /// Example 3:
    ///     Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
    ///     Output: [8,9,9,9,0,0,0,1]
    ///     Explanation: 9999999 + 9999 = 10009998
    ///
    /// Constraints:
    ///     - The number of nodes in each linked list is in the range [1, 100].
    ///     - `0 <= Node.val <= 9`
    ///     - It is guaranteed that the list represents a number that does not have leading zeros.
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_numbers(l1, l2, 0)
    }

    fn add_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => {
                if carry > 0 {
                    Some(Box::new(ListNode::new(carry)))
                } else {
                    None
                }
            }
            (Some(n), None) => Self::sum(n.val, 0, carry, n.next, None),
            (None, Some(n)) => Self::sum(0, n.val, carry, None, n.next),
            (Some(n1), Some(n2)) => Self::sum(n1.val, n2.val, carry, n1.next, n2.next),
        }
    }

    fn sum(
        value_1: i32,
        value_2: i32,
        carry: i32,
        next_1: Option<Box<ListNode>>,
        next_2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let new_value = value_1 + value_2 + carry;

        let (new_rest, new_value) = if new_value > 9 {
            (1, new_value - 10)
        } else {
            (0, new_value)
        };
        let mut result_node = ListNode::new(new_value);

        result_node.next = Self::add_numbers(next_1, next_2, new_rest);

        Some(Box::new(result_node))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use rstest::rstest;

    #[rstest]
    #[case((vec![2,4,3], vec![5,6,4]), &[7,0,8])]
    #[case((vec![0], vec![0]), &[0])]
    #[case((vec![9,9,9,9,9,9,9], vec![9,9,9,9]), &[8,9,9,9,0,0,0,1])]
    fn test_add_two_numbers(#[case] (l1, l2): (Vec<i32>, Vec<i32>), #[case] expected: &[i32]) {
        assert_eq!(
            Solution::add_two_numbers(Some(Box::new(l1.into())), Some(Box::new(l2.into())))
                .unwrap()
                .to_vec(),
            expected
        );
    }
}
