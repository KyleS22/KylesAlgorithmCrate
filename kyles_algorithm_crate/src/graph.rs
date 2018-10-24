
/// Adjacency list implementation of a graph
pub struct Graph<T>{
    nodes: Vec<Node<T>>,
}

/// Nodes that store data
pub struct Node<T>{
    data: T,
    edges: Vec<Node<T>>,
}

impl<T> Graph<T>{

    /// Create a new graph with no vertices
    pub fn new() -> Self{
        Graph{nodes: Vec::new()}
    }

    /// Add a new vertex to the graph with data
    ///
    /// # Arguments
    ///
    /// * `data` - The data to store at this vertex
    ///
    /// # Example
    ///
    /// ```
    /// // Usage of postorder traversal
    /// # use kyles_algorithm_crate::graph::Graph;
    /// // Create a tree with value 1 at its root
    /// let mut graph = Graph::new();    
    /// graph.add_vertex(1);
    /// ```
    pub fn add_vertex(&mut self, data: T){
        let vertex = Node{data: data, edges: Vec::new()};
        self.nodes.push(vertex);
    }

    pub fn add_directed_edge(&mut self, mut from: Node<T>, to: Node<T>){
        from.edges.push(to);
    }

}

#[cfg(test)]
mod tests {

}