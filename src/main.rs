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

impl ListNode {
    /// creates a linked list from a vector
    fn from_vec(mut vals: Vec<i32>) -> Self {
        // base case: create a base list
        let mut list = Self::new(*(vals.last().unwrap()));
        vals = vals[..vals.len() - 1].to_vec();
        while !vals.is_empty() {
            let mut new_list = Self::new(*(vals.last().unwrap()));
            new_list.next = Some(Box::new(list));
            list = new_list;
            vals.truncate(vals.len() - 1);
        }
        list
    }

    fn str(&self) -> String {
        let mut vals = Vec::<String>::new();
        let mut current = Some(Box::new(self.clone().to_owned()));
        while current.is_some() {
            let val = current.as_ref().unwrap().val.to_string();
            vals.push(val);
            current = current.unwrap().next;
        }

        format!("[{}]", vals.join(", "))
    }
}

// impl Clone for ListNode {
//     fn clone(&self) -> Self {
//         ListNode {
//             val: self.val,
//             next: None,
//         }
//     }
// }

fn pick_min_and_advance(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> (
    Option<Box<ListNode>>,
    Option<Box<ListNode>>,
    Option<Box<ListNode>>,
) {
    if list1.is_none() && list2.is_some() {
        let head = Some(Box::new(ListNode::new(list2.as_ref().unwrap().val)));
        return (head, None, list2);
    } else if list2.is_some() {
        if list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val {
            let head = Some(Box::new(ListNode::new(list1.as_ref().unwrap().val)));
            return (head, list1.unwrap().next, list2);
        }
        let head = Some(Box::new(ListNode::new(list2.as_ref().unwrap().val)));
        return (head, list1, list2.unwrap().next);
    }

    (None, None, None)
}

fn main() {
    let list1 = ListNode::from_vec(vec![4, 3, 5, 2, 6]);
    let list2 = ListNode::from_vec(vec![1, 8, 7]);
    println!("List 1 (original): {}", list1.str());
    println!("List 2 (original): {}", list2.str());

    println!();
    let (min, list1, list2) = pick_min_and_advance(Some(Box::new(list1)), Some(Box::new(list2)));
    println!("List 1 (result): {:?}", list1);
    println!("List 2 (result): {:?}", list2);

    println!();
    println!("Min chosen: {:?}", min);

    // println!("Hello, world!");
}
