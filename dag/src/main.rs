use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    id: usize,
    name: String,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            name: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct DAG {
    pub nodes: HashMap<usize, Node>,
    pub edges: Vec<(usize, usize)>,
}

impl DAG {
    pub fn new() -> Self {
        DAG {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_id: usize, name: String) {
        self.nodes.insert(node_id, Node { id: node_id, name });
    }

    pub fn add_edge(&mut self, from_node: usize, to_node: usize) {
        self.edges.push((from_node, to_node));
    }
}

fn main() {
    let mut dag = DAG::new();

    // 添加节点
    dag.add_node(1, "1".to_owned());
    dag.add_node(2, "2".to_owned());
    dag.add_node(3, "3".to_owned());
    dag.add_node(4, "4".to_owned());

    // 添加边
    dag.add_edge(1, 2);
    dag.add_edge(1, 3);
    dag.add_edge(2, 4);
    dag.add_edge(3, 4);

    println!("{:#?}", dag);
}
