

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
    
    /// Returns a new instance of a tree, with root_val as the value for the node
    ///
    /// # Arguments
    ///
    /// * `root_val` - The value to store at the root of this tree
    ///
    /// # Example
    ///
    /// ```
    /// # use kyles_algorithm_crate::tree::Tree;
    /// // Creates a new tree with the value 1 at its root
    /// let tree = Tree::new(1);
    /// ``` 
    pub fn new(root_val: T) -> Self {
        Tree { data: root_val, _parent: None, children: Vec::new()}
    }

    /// Adds a given tree to the list of children of this tree
    ///
    /// # Arguments
    /// 
    /// * `tree` - The tree to add to the list of children
    ///
    /// # Example
    ///
    /// ```
    /// # use kyles_algorithm_crate::tree::Tree;
    /// // Creates a new tree with the value 1 at its root
    /// let mut tree = Tree::new(1);
    ///
    /// let two = Tree::new(2);
    /// let three = Tree::new(3);
    ///
    /// // Add the two new trees to the root
    /// tree.add_subtree(two);
    /// tree.add_subtree(three);
    ///
    /// // root.children now contains two and three
    /// ```
    pub fn add_subtree(&mut self, tree: Tree<T>) {
        self.children.push(tree);
    }

    /// Add a new subtree to the root by specifying a value to store at the new child node
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store at the new child node
    ///
    /// # Example
    ///
    /// ```
    /// # use kyles_algorithm_crate::tree::Tree;
    /// // Creates a new tree with the value 1 at its root
    /// let mut tree = Tree::new(1);
    /// 
    /// tree.add_subtree_value(2);
    /// tree.add_subtree_value(3);
    ///
    /// // root.children now contains subtrees with values 2 and 3
    /// ```
    pub fn add_subtree_value(&mut self, value: T){
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

    /// Returns the values in the tree that are visited in a post-order traversal of the tree from the given node
    /// # Arguments
    /// * `start_node` - The sub tree to start the traversal at
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

    /// Returns the values in the tree that are visited in a pre-order traversal of the tree from the given node
    /// # Arguments
    /// * `start_node` - The sub tree to start the traversal at
    pub fn preorder_traversal_node(start_node: &Tree<T>) -> Vec<T>{
        let mut values: Vec<T> = Vec::new();
        values.push(start_node.data.clone());
    
        // for child in children, values.append(postorder_traversal(child))
        for child in start_node.children.iter() {
            values.append(&mut Tree::preorder_traversal_node(&child));
        }
    

        values        
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
    /// // Usage of preorder traversal
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
    /// let postorder = root.preorder_traversal();
    /// 
    pub fn preorder_traversal(&self) -> Vec<T> {
        let mut values: Vec<T> = Vec::new();
        values.append(&mut Tree::preorder_traversal_node(self));
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

    #[test]
    fn test_preorder_traversal(){
        use tree::Tree;

        let mut root = Tree::new(1);

        let mut result;
        result  = root.preorder_traversal();
        
        let mut expected = Vec::new();
        expected.push(1);

        assert_eq!(expected, result);

        let two = Tree::new(2);
        let three = Tree::new(3);
        let four = Tree::new(4);

        root.add_subtree(two);
        root.add_subtree(three);
        root.add_subtree(four);

        result = root.preorder_traversal();

        let mut expected = Vec::new();
        expected.push(1);
        expected.push(2);
        expected.push(3);
        expected.push(4);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_add_subtree(){
        use tree::Tree;

        let mut root = Tree::new(1);

        let two = Tree::new(2);
        let three = Tree::new(3);
        let four = Tree::new(4);

        root.add_subtree(two);
        root.add_subtree(three);
        root.add_subtree(four);

        assert_eq!(root.children[0].data, 2);
        assert_eq!(root.children[1].data, 3);
        assert_eq!(root.children[2].data, 4);
    }

    #[test]
    fn test_add_subtree_value(){
        use tree::Tree;

        let mut root = Tree::new(1);

        root.add_subtree_value(2);
        root.add_subtree_value(3);
        root.add_subtree_value(4);

        assert_eq!(root.children[0].data, 2);
        assert_eq!(root.children[1].data, 3);
        assert_eq!(root.children[2].data, 4);    }
}
