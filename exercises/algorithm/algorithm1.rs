/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
}

// 递归
/*
impl<T: PartialOrd + Copy> LinkedList<T> {
	pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> Self
	{
        if list_a.length == 0 {
            return list_b;
        }
        if list_b.length == 0 {
            return list_a;
        }
        let a_node = list_a.start;
        let b_node = list_b.start;
        let mut ans = LinkedList::<T>::new();
        let a_val = unsafe { (*a_node.unwrap().as_ptr()).val };
        let b_val = unsafe { (*b_node.unwrap().as_ptr()).val };
        if a_val < b_val {
            ans.add(a_val);
            let mut list_a = list_a;
            list_a.length -= 1;
            list_a.start = unsafe { (*a_node.unwrap().as_ptr()).next };
            let list_c = Self::merge(list_a, list_b);
            ans.length = list_c.length + 1;
            unsafe { (*ans.start.unwrap().as_ptr()).next = list_c.start };
        } else {
            ans.add(b_val);
            let mut list_b = list_b;
            list_b.length -= 1;
            list_b.start = unsafe { (*b_node.unwrap().as_ptr()).next };
            let list_c = Self::merge(list_a, list_b);
            ans.length = list_c.length + 1;
            unsafe { (*ans.start.unwrap().as_ptr()).next = list_c.start };
        }
        ans
	}
}
*/

// 迭代
impl<T: PartialOrd + Copy> LinkedList<T> {
	pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> Self
	{
        let mut a_node = list_a.start;
        let mut b_node = list_b.start;
        let mut ans = LinkedList::<T>::new();
        while a_node.is_some() && b_node.is_some() {
            let a_val = unsafe { a_node.unwrap().as_ref().val };
            let b_val = unsafe { b_node.unwrap().as_ref().val };
            if (a_val < b_val) {
                ans.add(a_val);
                a_node = unsafe { a_node.unwrap().as_ref().next };
            } else {
                ans.add(b_val);
                b_node = unsafe { b_node.unwrap().as_ref().next };
            }
        }
        if b_node.is_some() {
            std::mem::swap(&mut a_node, &mut b_node);
        }
        while a_node.is_some() {
            let a_val = unsafe { a_node.unwrap().as_ref().val };
            ans.add(a_val);
            a_node = unsafe { a_node.unwrap().as_ref().next };
        } 
        ans
	}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}