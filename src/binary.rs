use core::mem;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    NotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NotFound => f.write_str("NotFound"),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NotFound => "Record not found",
        }
    }
}

type NodeOption = Option<Box<Node>>;

#[derive(std::cmp::PartialEq)]
enum Direction {
    Left = 0,
    Right = 1,
}

#[derive(Debug, Default)]
struct Node {
    val: i32,
    left: NodeOption,
    right: NodeOption,
}
//
// impl Drop for Node {
//     fn drop(&mut self) {
//         println!("dropped Node -------------");
//     }
// }

impl Node {
    fn new(val: i32) -> Node {
        return Node {
            val,
            left: None,
            right: None,
        };
    }
}

#[derive(Debug)]
pub struct Tree {
    head: NodeOption
}

impl Tree {
    pub fn new() -> Tree {
        return Tree {
            head: None
        };
    }
    pub fn add(&mut self, val: i32) {
        let mut new_node = Node::new(val);
        Tree::add_to(&mut new_node, &mut self.head);
    }
    fn add_to(new_node: &mut Node, to_node: &mut NodeOption) {
        if to_node.is_none() {
            *to_node = Some(Box::new(mem::take(new_node)));
        } else {
            if new_node.val > to_node.as_ref().unwrap().val {
                Tree::add_to(&mut mem::take(new_node), &mut to_node.as_mut().unwrap().right);
            } else {
                Tree::add_to(&mut mem::take(new_node), &mut to_node.as_mut().unwrap().left);
            }
        }
    }

    pub fn remove(&mut self, val: i32) {
        match Tree::find(&mut self.head, val) {
            Err(err) => eprint!("Failed {:?}", err),
            Ok(node) => {
                println!("Found {:?}", node);
                Tree::replace_mut(node);
            }
        }
    }
    fn find1(node: &mut NodeOption, val: i32) -> Result<&mut NodeOption, Error> {
        if let Some(node_box) = node {
            if node_box.val == val {
                return Ok(node);
            }
        }

        match node {
            Some(node_box) => {
                match val.cmp(&node_box.val) {
                    Ordering::Less => Tree::find1(&mut node_box.left, val),
                    Ordering::Greater => Tree::find1(&mut node_box.right, val),
                    _ => Err(Error::NotFound),
                }
            }
            None => Err(Error::NotFound)
        }
    }

    fn find(node: &mut NodeOption, val: i32) -> Result<&mut NodeOption, Error> {
        if let Some(temp_val) = node.as_mut() {
            if temp_val.val == val {
                Ok(node)
            } else if temp_val.val < val {
                Tree::find(&mut node.as_mut().unwrap().right, val)
            } else {
                Tree::find(&mut node.as_mut().unwrap().left, val)
            }
        } else {
            Err(Error::NotFound)
        }
    }

    fn replace_mut(node: &mut NodeOption) {
        match (node.as_mut().unwrap().left.take(),
               node.as_mut().unwrap().right.take()) {
            (Some(child_left), None) => {
                node.replace(child_left);
            }
            (None, Some(child_right)) => {
                node.replace(child_right);
            }
            (None, None) => {
                node.take();
            }
            (Some(child_left), Some(mut child_right)) => {
                if child_right.left.is_none() {
                    child_right.left.replace(child_left);
                    node.replace(child_right);
                } else {
                    {
                        let mut smallest = child_right.left.as_mut();
                        while let Some(mut _box) = smallest {
                            if _box.left.is_none() {
                                let mut mut_box = mem::take(_box.as_mut());
                                mut_box.left.replace(child_left);
                                mut_box.right.replace(child_right);
                                node.replace(Box::from(mut_box));
                                break;
                            }
                            smallest = _box.left.as_mut();
                        };
                    }
                }
            }
        }
    }

    pub fn print(&self) {
        let mut queue: VecDeque<&NodeOption> = VecDeque::new();
        queue.push_back(&self.head);
        while !queue.is_empty() {
            let mut temp_queue: VecDeque<&NodeOption> = VecDeque::new();
            while !queue.is_empty() {
                let print_node = queue.pop_front().unwrap();
                if let Some(curr) = print_node {
                    print!("  - {} -  ", curr.val);
                    if curr.left.is_some() {
                        temp_queue.push_back(&curr.left);
                    }
                    if curr.right.is_some() {
                        temp_queue.push_back(&curr.right);
                    }
                }
            }
            while !temp_queue.is_empty() {
                queue.push_back(temp_queue.pop_front().unwrap());
            }
            println!();
        }
        println!();
    }
}
