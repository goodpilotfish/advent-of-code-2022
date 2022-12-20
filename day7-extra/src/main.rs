
// Tutorial from:
// https://applied-math-coding.medium.com/a-tree-structure-implemented-in-rust-8344783abd75
// Leaks. Use Weak instead. See https://doc.rust-lang.org/std/rc/struct.Rc.html

use std::rc::Rc;
use std::cell::RefCell;

#[derive(PartialEq)]
struct TreeNode {
    value: Option<u32>,
    children: Vec<Rc<RefCell<TreeNode>>>,
    parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new() -> TreeNode {
        TreeNode {
            value: None,
            children: vec![],
            parent: None,
        }
    }

    pub fn add_child(&mut self, node: Rc<RefCell<TreeNode>>) {
        self.children.push(node);
    }

    pub fn print(&self) -> String {
        if let Some(val) = self.value {
            val.to_string()
        } else {
            String::from("[") + &self
                .children
                .iter()
                .map(|tn| tn.borrow().print())
                .collect::<Vec<String>>()
                .join(",") + "]"
        }
    }
}

fn init_tree(s: String) -> Rc<RefCell<TreeNode>> {
    let root = Rc::new(RefCell::new(TreeNode::new()));
    let mut current = Rc::clone(&root);
    let chars = s.chars().collect::<Vec<char>>();

    // filter out first [, and last ]
    for c in chars[1..chars.len()-1]
        .iter()
        {
            println!("{}", *c);
            if *c == '[' || c.is_numeric() {
                let child = Rc::new(RefCell::new(TreeNode::new()));
                current.borrow_mut().children.push(Rc::clone(&child));
                {
                    let mut mut_child = child.borrow_mut();
                    mut_child.parent = Some(Rc::clone(&current));
                    if c.is_numeric() {
                      mut_child.value = c.to_digit(10);
                    }
                }
                current = child;
            } else if *c == ',' || *c == ']' {
                let current_clone = Rc::clone(&current);
                current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
            } else {
                panic!("Unknown character: {}", c);
            }
        }
        root
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_init_tree_1() {
    let tree = init_tree(String::from("[1,2]"));
    assert_eq!(tree.borrow().children[0].borrow().value.unwrap(), 1);
    assert_eq!(tree.borrow().children[1].borrow().value.unwrap(), 2);
  }

  #[test]
  fn test_init_tree_2() {
    let tree = init_tree(String::from("[1,2]"));
    assert_eq!(tree.borrow().children[1].borrow().value.unwrap(), 2);
  }

  #[test]
  fn test_init_tree_3() {
    let tree = init_tree(String::from("[0,1,[3,4,5,[7,8]],2]"));
    assert_eq!(tree.borrow().print(), "[0,1,[3,4,5,[7,8]],2]");
  }

  #[test]
  fn test_add_child() {
    let tree = init_tree(String::from("[0,1,[3,4,5,[7,8]],2]"));
    let new_node = Rc::new(RefCell::new(TreeNode::new()));
    new_node.borrow_mut().value = Some(9);
    let child = &tree.borrow().children[2];
    child.borrow_mut().add_child(new_node);
    assert_eq!(tree.borrow().print(), "[0,1,[3,4,5,[7,8],9],2]");
  }
}
