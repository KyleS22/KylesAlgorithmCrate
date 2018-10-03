

/// Representation of a tree structure
pub struct Tree<T> {
    /// The data stored at this node in the tree
    data: T,
    /// Reference to a parent tree
    _parent: Option<Box<Tree<T>>>,
    /// List of children trees
    children: Vec<Tree<T>>,
}

impl<T> Tree<T> 
    where T: Clone {

    pub fn new(root_val: T) -> Self {
        Tree { data: root_val, _parent: None, children: Vec::new()}
    }

    pub fn add_subtree(&mut self, tree: Tree<T>) {
        self.children.push(tree);
    }

    pub fn add_subtree_value(mut self, value: T){
        let subtree = Tree::new(value);
        self.add_subtree(subtree);
    }
    
    /// Returns the values in the tree that are visited in a post-order traversal of the tree from the root
    ///
    /// # Arguments
    /// 
    /// * `&self` - Borrows itself so that we can evaluate the child nodes and still be able to use the
    ///             reference that called this function afterward
    ///
    /// # Example
    ///
    /// ```
    /// // Usage of postorder traversal
    /// # use kyles_algorithm_crate::tree::Tree;
    /// # // Create a tree with value 1 at its root
    /// # let mut root = Tree::new(1);    
    ///
    /// # let two = Tree::new(2);
    /// # let three = Tree::new(3);
    /// # let four = Tree::new(4);
    ///
    /// # root.add_subtree(two);
    /// # root.add_subtree(three);
    /// # root.add_subtree(four);
    ///
    /// // This will contain a vector of the values in the order they were visited
    /// let postorder = root.postorder_traversal();
    /// 
    pub fn postorder_traversal(&self) -> Vec<T> {
        let mut values: Vec<T> = Vec::new();

        values.append(&mut Tree::postorder_traversal_node(self));
        values
    }

    pub fn postorder_traversal_node(start_node: &Tree<T>) -> Vec<T> {
        let mut values: Vec<T> = Vec::new();
        if start_node.children.len() == 0 {
            values.push(start_node.data.clone());
        } else { 
         // for child in children, values.append(postorder_traversal(child))
         for child in start_node.children.iter() {
             values.append(&mut Tree::postorder_traversal_node(&child));
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

        let mut result;
        result  = root.postorder_traversal();
        
        let mut expected = Vec::new();
        expected.push(1);

        assert_eq!(expected, result);

        let two = Tree::new(2);
        let three = Tree::new(3);
        let four = Tree::new(4);

        root.add_subtree(two);
        root.add_subtree(three);
        root.add_subtree(four);

        result = root.postorder_traversal();

        let mut expected = Vec::new();
        expected.push(2);
        expected.push(3);
        expected.push(4);
        expected.push(1);

        assert_eq!(expected, result);
    }
}
