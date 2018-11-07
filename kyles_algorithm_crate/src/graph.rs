use std::collections::HashMap;

/// Adjacency list implementation of a graph
pub struct Graph<T>{
    pub nodes: HashMap<u32, Node<T>>
}

/// Nodes that store data
pub struct Node<T>{
    data: T,
    edges: Vec<Edge>
}

/// Directed edges between nodes
pub struct Edge{
    from: u32, 
    to: u32,
    weight: u32
}

impl<T> Graph<T>{

    pub fn new() -> Self{
        Graph{nodes: HashMap::new()}
    }

    pub fn add_node(&mut self, node: Node<T>) -> u32{

        // The new node id will be the current number of nodes in the graph
        let node_id = self.nodes.len() as u32;

        // Insert the node
        self.nodes.insert(node_id, node);

        // Return the hash index of the inserted node
        node_id
    }

    pub fn add_directed_edge(from: u32, to: u32){
        // Create a new edge with weight 1
        // Get the from node from the hashmap
        // Add the edge to the node
    }

    pub fn add_undirected_edge(node1: u32, node2: u32){
        // Add directed edge from node1 to node2
        // Add directed edge from node2 to node1
    }

    pub fn add_weighted_directed_edge(from: u32, to: u32){
        // Create a new edge
        // Get the from node from the hashmap
        // Add the edge to the node
    }

    pub fn add_weighted_undirected_edge(node1: u32, to: u32){
        // Add directed edge from node1 to node2
        // Add directed edge from node2 to node1
    }
}

impl<T> Node<T>{

    /// Returns a new instance of a Node with the given data
    /// # Arguments
    ///
    /// * `data` - The data to store in the node 
    ///
    /// # Example
    ///
    /// ```
    /// # use kyles_algorithm_crate::graph::Graph;
    /// // Creates a new node
    /// let node = Node::new(10);
    /// ``` 
    pub fn new(data: T) -> Self{
        Node{data: data, edges: Vec::new()}
    }
}

impl Edge{

    /// Returns a new instance of an edge from one node to another, with a given weight
    /// # Arguments
    ///
    /// * `from` - The id of the node this edge starts from
    /// * `to`   - The id of the node this edge goes to
    /// * `weight` - The weight of the edge 
    ///
    /// # Example
    ///
    /// ```
    /// # use kyles_algorithm_crate::graph::Graph;
    /// // Creates a new edge
    /// let edge = Edge::new(1, 2, 10);
    /// ``` 
    pub fn new(from: u32, to: u32, weight: u32) -> Self{
        Edge{from: from, to: to, weight: weight}
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_add_node(){
        assert!(false);
    }

    #[test]
    fn test_add_directed_edge(){
        assert!(false);
    }

    #[test]
    fn test_add_undirected_edge(){
        assert!(false);
    }

    #[test]
    fn test_add_weighted_directed_edge(){
        assert!(false);
    }

    #[test]
    fn test_add_weighted_undirected_edge(){
        assert!(false);
    }
}