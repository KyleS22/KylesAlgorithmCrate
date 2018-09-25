

pub struct Tree<T> {
    data: T,
    _parent: Option<Box<Tree<T>>>,
    children: Vec<Tree<T>>,
}

impl<T> Tree<T> 
    where T: Clone {

    pub fn new(root_val: T) -> Self {
        Tree { data: root_val, _parent: None, children: Vec::new()}
    }

    pub fn postorder_traversal(self) -> Vec<T> {
        let mut values: Vec<T> = Vec::new();

        values.append(&mut Tree::postorder_traversal_node(self));
        values
    }

    pub fn postorder_traversal_node(start_node: Tree<T>) -> Vec<T> {
        let mut values: Vec<T> = Vec::new();
        if start_node.children.len() == 0 {
            values.push(start_node.data.clone());
        } else { 
         // for child in children, values.append(postorder_traversal(child))
         for child in start_node.children {
             values.append(&mut Tree::postorder_traversal_node(child));
         }
         values.push(start_node.data.clone());
        }
        values   
    }

    pub fn preorder_traversal(&self) -> Vec<T> {
        let values: Vec<T> = Vec::new();
        values
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_postorder_traversal() {
        use tree::Tree;

        let mut root = Tree::new(1);

        let result = root.postorder_traversal();
        
        let mut expected = Vec::new();
        expected.push(1);

        assert_eq!(expected, result);

        let two = Tree::new(2);
        let three = Tree::new(3);
        let four = Tree::new(4);

        root.children.push(two);
        root.children.push(three);
        root.children.push(four);

        let result = root.postorder_traversal();

        let mut expected = Vec::new();
        expected.push(2);
        expected.push(3);
        expected.push(4);
        expected.push(1);

        assert_eq!(expected, result);
    }
}
