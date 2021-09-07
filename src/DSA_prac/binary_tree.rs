use std::collections::LinkedList;

#[derive(Clone)]
pub struct Tree {
    pub val: i32,
    pub left: Option<Box<Tree>>,
    pub right: Option<Box<Tree>>,
}

impl Tree {
    fn new(val: i32) -> Self {
        Tree {
            val,
            left: None,
            right: None,
        }
    }

    fn add(&mut self, val: i32) {
        if val > self.val {
            match self.right {
                Some(ref mut r_child) => {
                    r_child.add(val);
                }
                None => {
                    self.right = Some(Box::new(Tree::new(val)));
                }
            }
        } else if val < self.val {
            match self.left {
                Some(ref mut l_child) => {
                    l_child.add(val);
                }
                None => {
                    self.left = Some(Box::new(Tree::new(val)));
                }
            }
        }
    }

    fn preOrder(&mut self) {
        print!("{}", self.val);
        match self.right {
            Some(ref mut r_child) => {
                r_child.preOrder();
            }
            None => {}
        }
        match self.left {
            Some(ref mut l_child) => {
                l_child.preOrder();
            }
            None => {}
        }
    }

    fn inOrder(&mut self) {
        match self.right {
            Some(ref mut r_child) => {
                r_child.preOrder();
            }
            None => {}
        }
        print!("{}", self.val);
        match self.left {
            Some(ref mut l_child) => {
                l_child.preOrder();
            }
            None => {}
        }
    }

    fn postOrder(&mut self) {
        match self.right {
            Some(ref mut r_child) => {
                r_child.preOrder();
            }
            None => {}
        }
        match self.left {
            Some(ref mut l_child) => {
                l_child.preOrder();
            }
            None => {}
        }
        print!("{}", self.val);
    }

    fn BFS(&mut self) {
        let mut q = LinkedList::new();
        q.push_front(self);
        while !q.is_empty() {
            let mut temp_tree = q.pop_front().unwrap();
            print!("{}", temp_tree.val);
            match temp_tree.left {
                Some(ref mut l_child) => {
                    q.push_back(l_child);
                }
                None => {}
            }
            match temp_tree.right {
                Some(ref mut r_child) => {
                    q.push_back(r_child);
                }
                None => {}
            }
        }
    }

    fn DFS(&mut self) {
        let mut s = LinkedList::new();
        s.push_back(self);
        while !s.is_empty() {
            let mut temp_tree = s.pop_back().unwrap();
            print!("{}", temp_tree.val);
            match temp_tree.right {
                Some(ref mut r_child) => {
                    s.push_back(r_child);
                }
                None => {}
            }
            match temp_tree.left {
                Some(ref mut l_child) => {
                    s.push_back(l_child);
                }
                None => {}
            }
        }
    }

    fn find_max_depth(&self) -> usize {
        let mut tall;
        let mut left;
        let mut right;

        match self.left {
            Some(ref t) => {
                left = t.find_max_depth();
            }
            None => {
                left = 0;
            }
        }

        match self.right {
            Some(ref t) => {
                right = t.find_max_depth();
            }
            None => {
                right = 0;
            }
        }

        if left > right {
            tall = left + 1;
        } else if right > left {
            tall = right + 1;
        } else {
            tall = right + 1;
        }

        return tall;
    }

    fn find_min_depth(&self) -> usize {
        let mut tall;
        let mut left;
        let mut right;

        match self.left {
            Some(ref t) => {
                left = t.find_min_depth();
            }
            None => {
                left = 0;
            }
        }

        match self.right {
            Some(ref t) => {
                right = t.find_min_depth();
            }
            None => {
                right = 0;
            }
        }

        if left > right {
            if right == 0 {
                tall = left + 1;
            } else {
                tall = right + 1;
            }
        } else if right > left {
            if left == 0 {
                tall = right + 1;
            } else {
                tall = left + 1;
            }
        } else {
            tall = right + 1;
        }
        return tall;
    }
}

fn main() {
    let mut t = Tree::new(0);
}
