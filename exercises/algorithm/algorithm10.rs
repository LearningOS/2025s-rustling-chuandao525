use std::collections::{HashMap, HashSet};
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
    
    // 修正参数列表的括号闭合问题
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (node_a, node_b, weight) = edge;
        
        // 确保两个节点存在
        self.add_node(node_a);
        self.add_node(node_b);
        
        // 获取邻接表的可变引用
        let adj = self.adjacency_table_mutable();
        
        // 添加双向边
        adj.get_mut(node_a).unwrap().push((node_b.to_string(), weight));
        adj.get_mut(node_b).unwrap().push((node_a.to_string(), weight));
    }
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    
    fn add_node(&mut self, node: &str) -> bool {
        if self.contains(node) {
            false
        } else {
            self.adjacency_table_mutable()
                .insert(node.to_string(), Vec::new());
            true
        }
    }
    
    fn add_edge(&mut self, edge: (&str, &str, i32));
    
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, neighbors) in self.adjacency_table() {
            for (to_node, weight) in neighbors {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::*;
    
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
        ];
        
        for edge in expected_edges.iter() {
            assert!(graph.edges().contains(edge));
        }
    }
}