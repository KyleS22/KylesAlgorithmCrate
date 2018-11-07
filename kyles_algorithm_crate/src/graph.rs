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

    /// Returns a new instance of a Graph
    /// # Example
    ///
    /// ```
    /// # use kyles_algorithm_crate::graph::Graph;
    /// // Creates a new graph
    /// let graph = Graph::new(10);
    /// ``` 
    pub fn new() -> Self{
        Graph{nodes: HashMap::new()}
    }

    /// Add a new node to the graph with supplied data.  Returns the id of the node in the graph.
    /// # Arguments
    /// 
    /// `data` - The data to put in the node to be added
    ///
    /// # Example
    ///
    /// ```
    /// # use kyles_algorithm_crate::graph::Graph;
    /// // Creates a new node
    /// let graph = Graph::new();
    /// 
    /// // Adds a new node with value 10 to the graph
    /// // node_id now contains the id to access that node in the graph
    /// let node_id = graph.add_node(10);
    /// 
    /// ``` 
    pub fn add_node(&mut self, data: T) -> u32{

        // The new node id will be the current number of nodes in the graph
        let node_id = self.nodes.len() as u32;

        let new_node = Node::new(data);

        // Insert the node
        self.nodes.insert(node_id, new_node);

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
        use graph::Graph;

        let mut graph = Graph::new();

        let zeroth_node = graph.add_node(0);

        assert_eq!(zeroth_node, 0);
        assert_eq!(graph.nodes.len(), 1);
        // TODO: FIXassert_eq!(graph.nodes.get(&0), Some(Node.data));

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