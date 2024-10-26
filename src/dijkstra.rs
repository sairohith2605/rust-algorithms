use std::collections::BinaryHeap;

struct Node {
    name: String,
    weight: u16,
}

struct Graph<'a> {
    source_node: &'a Node,
    nodes: Vec<Node>,
}

pub fn compute_shortest_path(graph: &[Graph], source: &Node) {
    let mut distance: Vec<i32> = vec![-1; graph.len()];
    let mut path_queue: BinaryHeap<&Node> = BinaryHeap::new();
    distance[0] = 0;
    path_queue.push(source);

    while !path_queue.is_empty() {
        let current_node = graph.iter()
            .find(|&&node| node.source_node.name == path_queue.peek().unwrap().name)
            .unwrap();
        // let (x, y) = (current_node., current_node.source_node.name);
        
    }
}

#[cfg(test)]
mod tests {
    use crate::dijkstra::{compute_shortest_path, Graph, Node};

    #[test]
    fn test_shortest_path() {
        /*
        A: [(B, 5), (C, 3)]
        B: [(C, 6)]
        C: [(B, 6)]
        */
        let node_a = Node {
            name: String::from("A"),
            weight: 0,
        };
        let node_b = Node {
            name: String::from("B"),
            weight: 0,
        };
        let node_c = Node {
            name: String::from("C"),
            weight: 0,
        };
        let graph: [Graph; 3] = [
            Graph {
                source_node: &node_a,
                nodes: vec![
                    Node {
                        name: node_b.name,
                        weight: 5,
                    },
                    Node {
                        name: node_c.name,
                        weight: 3,
                    },
                ],
            },
            Graph {
                source_node: &node_b,
                nodes: vec![Node {
                    name: node_c.name,
                    weight: 6,
                }],
            },
            Graph {
                source_node: &node_c,
                nodes: vec![Node {
                    name: node_b.name,
                    weight: 6,
                }],
            },
        ];

        compute_shortest_path(&graph, &node_a);
    }
}
