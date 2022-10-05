use std::collections::{VecDeque, HashMap};

type NodeIndex = usize;
type EdgeIndex = usize;

#[derive(Clone, Copy)]
struct NodeData {
    first_outgoing: Option<usize>
}
#[derive(Clone, Copy)]
struct EdgeData {
    target: NodeIndex,
    next_outgoing: Option<EdgeIndex>,
    weight: u32
}
struct Graph {
    pub nodes: Vec<NodeData>,
    pub edges: Vec<EdgeData>
}

impl Graph {
    pub fn add_node(&mut self) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData {
            first_outgoing: None
        });
        index
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex, weight: u32) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            target: target,
            next_outgoing: node_data.first_outgoing,
            weight: weight
        });
        node_data.first_outgoing = Some(edge_index);
    }
}

struct Searcher {
    cur: NodeIndex,
    visited: u32,
    path_len: u32
}

impl Searcher {
    pub fn visited(&self, node: NodeIndex) -> bool {
        self.visited & (1 << node) != 0
    }
}

fn main() {
    let content = include_str!("../input.txt");
    let mut locations = HashMap::new();
    let mut g = Graph {
        nodes: Vec::new(),
        edges: Vec::new()
    };

    for l in content.lines() {
        let parts : Vec<&str> = l.split(" ").collect();
        let location_a = parts[0];
        let location_b = parts[2];
        let weight = parts[4].parse::<u32>().unwrap();

        if !locations.contains_key(location_a) {
            let idx = g.add_node();
            locations.insert(location_a, idx);
        }

        if !locations.contains_key(location_b) {
            let idx = g.add_node();
            locations.insert(location_b, idx);
        }

        let source = locations.get(location_a).unwrap();
        let dest = locations.get(location_b).unwrap();

        g.add_edge(*source, *dest, weight);
        g.add_edge(*dest, *source, weight);
    }

    let mut visited_all : u32 = 0x0;
    for i in 0..g.nodes.len() {
        visited_all |= 1 << i;
    }

    let mut search : VecDeque<Searcher> = VecDeque::new();
    for i in 0..g.nodes.len() {
        search.push_back(Searcher {
            cur: i,
            visited: 1 << i,
            path_len: 0x0
        });
    }

    let mut min_path_len = u32::MAX;
    let mut max_path_len = 0x0;

    while let Some(s) = search.pop_front() {
        let node = g.nodes[s.cur];

        if s.visited == visited_all {
            if s.path_len < min_path_len {
                min_path_len = s.path_len;
            }
            else if s.path_len > max_path_len {
                max_path_len = s.path_len;
            }
        }

        let mut idx = node.first_outgoing;
        while let Some(i) = idx {
            let e = g.edges[i];
            if !s.visited(e.target) {
                search.push_back(Searcher {
                    cur: e.target,
                    visited: s.visited | (1 << e.target),
                    path_len: s.path_len + e.weight
                });
            }

            idx = e.next_outgoing;
        }

    }

    println!("Min path: {}", min_path_len);
    println!("Max path: {}", max_path_len);
}
