use std::collections::HashMap;

/// Adjacency list implementation of a graph
pub struct Graph<T>{
    pub nodes: HashMap<u32, Node<T>>,
    pub edges: Vec<Edge>
}

/// Nodes that store data
pub struct Node<T>{
    data: T,
    edges: Vec<Node<T>>,
}

/// Directed edges between nodes
pub struct Edge{
    from: u32,
    to: u32,
    weight: u32
}

impl<T> Graph<T>{

    pub fn new() -> Self{
        Graph{nodes: HashMap::new(), edges: Vec::new()}
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

    }

    pub fn add_undirected_edge(node1: u32, node2: u32){

    }

    pub fn add_weighted_directed_edge(from: u32, to: u32){

    }

    pub fn add_weighted_undirected_edge(node1: u32, to: u32){

    }
}

#[cfg(test)]
mod tests {

}