/*
	graph
	This problem requires you to implement a basic graph functio
*/


use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;


#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }

    fn add_node(&mut self, node: &str) -> bool {
        if self.contains(node) {
            return false; // Node already exists
        }
        self.adjacency_table.insert(node.to_string(), Vec::new());
        true
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (from, to, weight) = edge;

        // Ensure both nodes exist in the graph
        self.add_node(from);
        self.add_node(to);

        // Add the edge in both directions (undirected graph)
        self.adjacency_table_mutable()
            .entry(from.to_string())
            .or_insert_with(Vec::new)
            .push((to.to_string(), weight));
        self.adjacency_table_mutable()
            .entry(to.to_string())
            .or_insert_with(Vec::new)
            .push((from.to_string(), weight));
    }

    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }

    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool;
    fn add_edge(&mut self, edge: (&str, &str, i32));
    fn contains(&self, node: &str) -> bool;
    fn nodes(&self) -> HashSet<&String>;
    fn edges(&self) -> Vec<(&String, &String, i32)>;
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    use std::collections::HashSet;

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];

        for edge in expected_edges.iter() {
            assert!(graph.edges().contains(edge), "Edge {:?} not found", edge);
        }
    }

    #[test]
    fn test_add_node() {
        let mut graph = UndirectedGraph::new();
        assert!(graph.add_node("a"));
        assert!(!graph.add_node("a")); // Should return false as "a" already exists
    }

    #[test]
    fn test_graph_contains() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        assert!(graph.contains("a"));
        assert!(graph.contains("b"));
        assert!(!graph.contains("c")); // "c" does not exist
    }

    #[test]
    fn test_graph_nodes() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        let nodes: HashSet<&String> = graph.nodes();
        assert!(nodes.contains(&"a".to_string()));
        assert!(nodes.contains(&"b".to_string()));
        assert!(nodes.contains(&"c".to_string()));
    }
}
