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



    fn print_in_order(node: &Option<Box<Node>>, level: usize){
        if let Some(n) = node{
            println!("Level {}: {}", level, n.data);
            Self::print_in_order(&n.left, level + 1);
            Self::print_in_order(&n.right, level + 1);
        }
    }

    pub fn print(&self){
        Self::print_in_order(&self.root, 1)
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    fn test_insert(){
        let mut bst1 = BinarySearchTree::new();
        bst1.insert(3);
        bst1.insert(2);
        bst1.insert(4);
        bst1.insert(9);
        bst1.insert(1);
        bst1.print();
    }
}


