#[allow(unused_assignments)]
pub mod linked_list {
    #[derive(Debug)]
    pub struct Node<T> {
        value: T,
        next: Option<Box<Node<T>>>
    }

    impl<T> Node<T> {
        pub fn new(value: T) -> Self {
            Node {
                value: value,
                next: None
            }
        }

        pub fn set_next(&mut self, node: Option<Box<Node<T>>>) {
            self.next = node;
        }

        pub fn get_next(&self) -> &Option<Box<Node<T>>> {
            &self.next
        }
    }

    pub struct LinkedList<T> {
        head: Option<Box<Node<T>>>,
        length: i32
    }

    impl<T: std::fmt::Debug> LinkedList<T> {
        pub fn new() -> Self {
            LinkedList {
                head: None,
                length: 0
            }
        }

        pub fn len(&self) -> i32 {
            self.length
        }

        pub fn push_front(&mut self, value: T) {
            let mut new_node = Box::new(Node::new(value));

            if let None = self.head {
                self.head = Some(new_node);
                self.length += 1;
                return;
            }

            new_node.set_next(self.head.take());
            self.head = Some(new_node);


            self.length += 1;
        }

        pub fn push_back(&mut self, value: T) {
            let new_node = Box::new(Node::new(value));

            if let None = self.head {
                self.head = Some(new_node);
                self.length += 1;
                return;
            }

            let mut temp = &mut self.head;

            while let Some(node) = temp {
                if let None = node.get_next() {
                    node.set_next(Some(new_node));
                    self.length += 1;
                    return;
                }

                temp = &mut node.next;
            }
        }

        pub fn get(&self, index: usize) -> Option<&T> {
            let mut temp = &self.head;
            let mut count = 0;

            if index >= self.length as usize {
                eprintln!("Index out of bounds");
                return None;
            }

            while let Some(node) = temp {
                if count == index {
                    return Some(&node.value);
                }

                count += 1;
                temp = &node.next;
            }

            None
        }

        pub fn pop_front(&mut self) -> Option<T> {
            if let None = self.head {
                return None;
            }

            let node = self.head.take();
            let node = node.unwrap();

            self.head = node.next;
            self.length -= 1;
            Some(node.value)
        }

        pub fn pop_back(&mut self) -> Option<&T> {
            if let None = self.head {
                return None;
            }

            let mut temp = &mut self.head;

            while let Some(node) = temp {
                if node.get_next().is_none() {
                    let node_value = &node.value;
                    temp = &mut node.next;
                    self.length -=1;
                    return Some(node_value);
                }

                temp = &mut node.next;
            }

            None
        }

        pub fn clear(&mut self) {
            self.head = None;
            self.length = 0;
        }

        pub fn print(&self) {
            let mut temp = &self.head;

            if let None = self.head {
                println!("List is empty");
                return;
            }

            while let Some(node) = temp {
                if let None = node.get_next() {
                    println!("{:?}", node.value);
                }
                else {
                    print!("{:?} -> ", node.value);
                }
                temp = &node.next;
            }

            println!("Total Length: {}", self.length);
        }
    }
}
