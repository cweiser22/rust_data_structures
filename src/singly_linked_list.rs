// represents one item in a singly-linked list
struct Node {
    data: String,
    next: Option<Box<Node>>,
}

impl Node {
    // constructor for a node
    fn new(data: String, next: Option<Box<Node>>) -> Self {
        Node { data, next }
    }
}

pub struct SinglyLinkedList {
    head: Option<Box<Node>>,
    length: usize,
}

impl SinglyLinkedList {
    pub fn new() -> Self {
        SinglyLinkedList{head: None, length: 0}
    }

    // inserts a String at the head
    pub fn push_head(&mut self, data: &str){
        let old_head = self.head.take();
        let new_node = Node::new(data.to_string(), old_head);
        self.head = Some(Box::new(new_node));
        self.length += 1
    }

    pub fn contains(&self, s: &str) -> bool{
        let mut current = &self.head;
        while let Some(node) = current{
            if node.data == s{
                return true;
            }
            current = &node.next
        }
        false
    }


    pub fn print_list(&mut self){
        let mut i: usize = 0;
        println!("List length: {}", self.length);
        if self.length > 0 {
            let mut current = &mut self.head;
            while let Some(node) = current {
                current = &mut node.next;
                println!("List element {}: {}", i, node.data);
                i += 1;
            }
        }
    }

    pub fn len(&self) -> usize{
        self.length
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn push_elements(){
        let mut l1 = SinglyLinkedList::new();
        l1.push_head("Hi");
        l1.push_head("Helllooooooo");
        l1.push_head("Helllooooooo22222");
        l1.push_head("Helllooooooo33333");

        assert_eq!(l1.length, 4);

        l1.print_list();
    }

    #[test]
    fn check_contains(){
        let mut l1 = SinglyLinkedList::new();
        assert_eq!(l1.contains("hello"), false);
        l1.push_head("Hi");
        assert_eq!(l1.contains("Hi"), true);
        assert_eq!(l1.contains("hi"), false);
        l1.push_head("hi");
        assert_eq!(l1.contains("hi"), true);
    }
}