fn main() {
    println!("Hello, world!");
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
   pub val: i32,
   pub next: Option<Box<ListNode>>
 }
 
 impl ListNode {
   #[inline]
   fn new(val: i32) -> Self {
     ListNode {
       next: None,
       val
     }
   }
 }

 pub fn merge_in_between(list1: Option<Box<ListNode>>, a: i32, b: i32, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    pub fn merge_in_between(list1: Option<Box<ListNode>>, a: i32, b: i32, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;

        if list1.is_none() || list2.is_none() {
            return None;
        }

        let mut curr = list1.as_mut().unwrap().as_mut();
        let mut start = curr;
        let mut end = curr;
        let mut count = 0;

        while let Some(node) = curr {
            count += 1;
            if count == a {
                start = node;
            }
            curr = node.next.as_mut().map(|n| n.as_mut());
            if count == b + 1 {
                end = curr.unwrap();
            }
        }

        let mut curr = list2.as_mut().unwrap().as_mut();
        start.next = list2;
        while let Some(node) = curr {
            if let Some(next) = &mut node.next {
                curr = next.as_mut();
            } else {
                break;
            }
        }

        curr.unwrap().next = end.next;
        list1
    } 
 }
