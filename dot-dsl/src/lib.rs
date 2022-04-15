

pub mod graph {

    use graph_items::node::Node as Node;
    use graph_items::edge::Edge;
    use std::collections::HashMap;

    pub mod graph_items {                                                                                     
        pub mod node {
            use std::collections::HashMap;
            #[derive(Clone, PartialEq, Eq, Debug)]

            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(self, attr: &[(&str, &str)]) -> Self {
                    let mut tmp: HashMap<String, String> = self.attrs.clone();

                    for x in attr {
                        let (a, b) = x;
                        tmp.insert(a.to_string(), b.to_string());
                    }

                    Node {
                        name: self.name,
                        attrs: tmp,
                    }
                }
                pub fn get_attr(&self,k : &str) -> Option<&str>{
                    self.attrs.get(k).map(|a| a.as_str())
                }
            }
        }

        pub mod edge {
            use std::collections::HashMap;
            #[derive(Clone, PartialEq, Eq, Debug)]
            pub struct Edge {
                nodes_linked: Vec<String>,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(a: &str, b: &str) -> Self {
                    Edge {
                        nodes_linked: Vec::from([a.to_string(), b.to_string()]),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(self, attr: &[(&str, &str)]) -> Self {
                    let mut tmp: HashMap<String, String> = self.attrs.clone();

                    for x in attr {
                        let (a, b) = x;
                        tmp.insert(a.to_string(), b.to_string());
                    }

                    Edge {
                        nodes_linked: self.nodes_linked,
                        attrs: tmp,
                    }
                }
            }
        }
    }

    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }
        pub fn with_nodes(mut self, nodes: &Vec<graph_items::node::Node>) ->  Self {
            self.nodes=nodes.to_vec();
            self
        }
        pub fn with_edges(mut self, edges: &Vec<graph_items::edge::Edge>) -> Self {
            self.edges=edges.to_vec();
            self
        }
        pub fn with_attrs(self, attr: &[(&str, &str)]) -> Self {
            let mut tmp: HashMap<String, String> = self.attrs.clone();

            for x in attr {
                let (a, b) = x;
               tmp.insert(a.to_string(), b.to_string());
            }

            Graph {
                nodes: self.nodes,
                edges: self.edges,
                attrs: tmp,
            }
        }
        pub fn get_node(& self,node_name:&str) -> Option<&Node>{
            for node in self.nodes.iter(){
                if node.name==node_name{
                    return Some(node);
                }
            }
            None
        }
    }
}
/*
//use graph::graph_items::edge::Edge;

use graph::Graph;

//use maplit::hashmap;

pub fn main() {
    let g = Graph::new();
}
*/