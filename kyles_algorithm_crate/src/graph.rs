use std::collections::HashMap;

/// Adjacency list implementation of a graph
pub struct Graph<T>{
    pub nodes: HashMap<u32, Node<T>>
}

#[derive(Debug)]
/// Nodes that store data
pub struct Node<T>{
    data: T,
    edges: Vec<Edge>
}

#[derive(Debug)]
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

    pub fn add_directed_edge(&mut self, from: u32, to: u32){
        // Create a new edge with weight 1
        let edge = Edge::new(from, to, 1);

        // Grab the from node and add the edge to its list of edges
        if let Some(n) = self.nodes.get_mut(&from){
            n.edges.push(edge);
        }
    }

    pub fn add_undirected_edge(&mut self, node1: u32, node2: u32){
        // Add directed edge from node1 to node2
        self.add_directed_edge(node1, node2);

        // Add directed edge from node2 to node1
        self.add_directed_edge(node2, node1);
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

    pub fn get_node(&mut self, node: u32) -> Option<&mut Node<T>>{

        self.nodes.get_mut(&node)

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
        assert_eq!(graph.nodes.get(&zeroth_node).unwrap().data, 0);
        
        let first_node = graph.add_node(1);

        assert_eq!(first_node, 1);
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.nodes.get(&zeroth_node).unwrap().data, 0);
        assert_eq!(graph.nodes.get(&first_node).unwrap().data, 1);


    }

    #[test]
    fn test_get_node(){
        assert!(false);
    }


    #[test]
    fn test_add_directed_edge(){
        use graph::Graph;

        let mut graph = Graph::new();

        let zeroth_node = graph.add_node(0);
        let first_node = graph.add_node(1);

        graph.add_directed_edge(zeroth_node, first_node);

        let graph_nodes = &mut graph.nodes;

        let node = &graph_nodes.get(&zeroth_node).unwrap();

        assert_eq!(node.edges.len(), 1);
        assert_eq!(node.edges[0].to, first_node);
        assert_eq!(node.edges[0].weight, 1);
        
        let node2 = graph_nodes.get(&first_node).unwrap();

        assert_eq!(node2.edges.len(), 0);

        let mut graph2 = Graph::new();

        let node1_id = graph2.add_node(1);
        let node2_id = graph2.add_node(2);


        graph2.add_directed_edge(node1_id, node2_id);
        graph2.add_directed_edge(node2_id, node1_id);

        let graph_nodes2 = &mut graph2.nodes;

        let node = &graph_nodes2.get(&node1_id).unwrap();

        assert_eq!(node.edges.len(), 1);
        assert_eq!(node.edges[0].to, node2_id);
        assert_eq!(node.edges[0].weight, 1);
        
        let node2 = graph_nodes2.get(&node2_id).unwrap();

        assert_eq!(node2.edges.len(), 1);
        assert_eq!(node2.edges[0].to, node1_id);
        assert_eq!(node2.edges[0].weight, 1);



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