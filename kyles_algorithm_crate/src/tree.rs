pub struct Tree<T> {
    data: T,
    parent: Box<Tree<T>>,
    children: Vec<Tree<T>>,
}

impl<T> Tree<T> 
    where T: Clone {

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
        assert!(false);
    }
}
