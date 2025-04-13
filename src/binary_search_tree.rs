use std::collections::VecDeque;
use std::collections::HashSet;

pub struct BinarySearchTree{
    root: Option<Box<Node>>,

}


struct Node{
    data: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

impl Node{
    fn new(data: i32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Node{
        Node{data, left, right}
    }
}

impl BinarySearchTree{
    pub fn new() -> BinarySearchTree{
        BinarySearchTree{root: None}
    }

    pub fn insert(&mut self, data: i32){
        match &mut self.root{
            Some(root) => {
                let mut current = &mut self.root;
                while let Some(parent) = current{
                    if parent.data == data{
                        // TODO: do nothing
                        return
                    } else if data < parent.data{
                        current = &mut parent.left;
                    } else if data > parent.data{
                        current = &mut parent.right;
                    } else {
                        panic!("This should never happen.")
                    }
                }
                *current = Some(Box::new(Node::new(data, None, None)))
            },
            None => {
                println!("Setting root!");
                self.root = Some(Box::new(Node::new(data, None, None)))
            }
        }
    }


    // prints a tree with bread-first search
    pub fn print_bfs(&self){
        let mut q = VecDeque::new();

        q.push_back(&self.root);


        while !q.is_empty(){
            let current = q.pop_front();
            match current{
                Some(Some(node)) => {
                    println!("{}", node.data);
                    q.push_back(&node.left);
                    q.push_back(&node.right);
                },
                Some(_) => {},
                None => {}
            }
        }
    }

    // prints a tree with depth-first search
    pub fn print_dfs(&self){
        let mut q = VecDeque::new();

        q.push_front(&self.root);


        while !q.is_empty(){
            let current = q.pop_front();
            match current{
                Some(Some(node)) => {
                    println!("{}", node.data);
                    q.push_front(&node.left);
                    q.push_front(&node.right);
                },
                Some(_) => {},
                None => {}
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_insert(){
        let mut bst1 = BinarySearchTree::new();
        bst1.insert(3);
        bst1.insert(2);
        bst1.insert(4);
        bst1.insert(9);
        bst1.insert(1);
        bst1.print_dfs();
        bst1.print_bfs();
    }
}


