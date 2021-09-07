use std::io;

#[derive(Clone)]
pub struct Node {
    data: i64,
    next: Option<Box<Node>>,
}

pub struct List {
    head: Option<Box<Node>>,
}

impl Node {
    fn new(data: i64) -> Self {
        Node { data, next: None }
    }
    fn print(&mut self) {
        println!("{}", self.data);
        match self.next {
            Some(ref mut ptr) => {
                ptr.print();
            }
            None => {}
        }
    }
}

impl List {
    fn new(data: i64) -> Self {
        List {
            head: Some(Box::new(Node::new(data))),
        }
    }
    fn add_front(&mut self, data: i64) {
        let mut temp = Node::new(data);
        temp.next = self.head;
        self.head = Some(Box::new(temp));
    }
    fn add_last(&mut self, data: i64) {
        match self.next {
            Some(ref mut ptr) => {
                ptr.add_last(data);
            }
            None => {
                self.next = Some(Box::new(Node::new(data)));
            }
        }
    }
    fn delete_front(&mut self) {
        match self.next {
            Some(ref mut ptr) => self = ptr,
            None => self = None,
        }
    }
    fn delete_last(&mut self) {
        match self.next {
            Some(ref mut ptr) => {
                ptr.delete_last();
            }
            None => {
                self = None;
            }
        }
    }
}

fn main() {
    let mut n = List::new(1);

    n.add_front(0);
    n.add_last(2);
    n.head.unwrap().as_deref().print();

    // let mut n = Node::new(1);
    // n.add_front(0);
    // n.add_last(2);
    // // n.delete_last();
    // // n.delete_front();
    // n.print();
}

//
// use std::io;
//
// #[derive(Clone)]
// pub struct Node {
//     data: i64,
//     next: Option<Box<Node>>,
// }
//
// pub struct List {
//     head: Option<Box<Node>>,
// }
//
// impl List {
//     fn new(data: i64) -> Self {
//         List {
//             head: Some(Box::new(Node { data, next: None })),
//         }
//     }
//     fn print(&mut self) {
//         let mut head_cpy = self.head.as_ref();
//         while let Some(ref mut ptr) = head_cpy {
//             println!("{}", ptr.data);
//             head_cpy = ptr.next.as_ref();
//         }
//     }
//     fn length(&mut self) -> i64 {
//         let mut cnt = 0;
//         let mut head_cpy = self.head.as_ref();
//         while let Some(ref mut ptr) = head_cpy {
//             cnt += 1;
//             head_cpy = ptr.next.as_ref();
//         }
//         cnt
//     }
//     fn get_nth_node_mut(&mut self, n: usize) -> Option<&mut Node> {
//         let mut nth_node = self.head.as_mut();
//         for _ in 0..n {
//             nth_node = match nth_node {
//                 None => return None,
//                 Some(node) => node.next.as_mut().map(|node| &mut *node),
//             }
//         }
//         nth_node
//     }
//     fn add_front(&mut self, data: i64) {
//         let mut temp = Node {
//             data,
//             next: self.head.take(),
//         };
//         self.head = Some(Box::new(temp));
//     }
//     fn add_last(&mut self, data: i64) {
//         let mut l = self.length();
//         let mut nth_node = self.head.as_mut();
//         let mut key = false;
//         for _ in 0..l {
//             nth_node = match nth_node {
//                 Some(node) => node.next.as_mut().map(|node| &mut *node),
//                 None => {
//                     key = true;
//                     Some(Box::new(Node { data, next: None }))
//                 }
//             };
//             if key {
//                 break;
//             }
//         }
//         // let mut head_cpy = self.head.as_ref();
//         // while let Some(ref ptr) = head_cpy.unwrap().next {
//         //     head_cpy = ptr.next.as_ref();
//         // }
//         // head_cpy.map(|node| node.next = Some(Box::new(Node { data, next: None })));
//         // head_cpy = Some(Box::new(Node { data, next: None }));
//     }
//     fn delete_front(&mut self) {
//         self.head = self.head.take().unwrap().next;
//     }
//     fn delete_last(&mut self) {
//         let mut head_cpy = self.head.as_ref();
//         while let Some(ref ptr) = head_cpy.unwrap().next {
//             head_cpy = ptr.next.as_ref();
//         }
//         head_cpy = None;
//     }
// }
//
// fn main() {
//     let mut n = List::new(1);
//     n.add_front(0);
//     let mut l = n.length();
//     // n.add_last(2);
//     // n.delete_last();
//     n.print();
//     println!("{}", l);
// }
