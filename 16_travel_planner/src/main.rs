use std::collections::{BinaryHeap, HashMap, HashSet};

type Node = usize;
type Cost = usize;

struct Graph {
    edges: HashMap<Node, Vec<(Node, Cost)>>,
    nodes: HashSet<Node>,
}

impl Graph {
    fn from_edge_list(edge_list: &Vec<(Node, Node, Cost)>) -> Self {
        let mut adjacency_list: HashMap<Node, Vec<(Node, Cost)>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list.entry(source).or_insert_with(|| Vec::new());

            destinations.push((destination, cost));

            nodes.insert(source);
            nodes.insert(destination);
        }

        Graph {
            edges: adjacency_list,
            nodes,
        }
    }
}

// need to create a priority queue (which we're doing with Step)
#[derive(PartialEq, Eq)]
struct Step {
    cost: Cost,
    position: Node,
    history: Vec<Node>,
}

impl Ord for Step {
    fn cmp(&self, other: &Step) -> std::cmp::Ordering {
        (other.cost)
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // example of partial compare for reference

        // match self.cost.partial_cmp(&other.cost) {
        //    Some(core::cmp::Ordering::Equal) => {}
        //    ord => return ord,
        //}
        //match self.cost.partial_cmp(&other.position) {
        //    Some(core::cmp::Ordering::Equal) => {}
        //    ord => return ord,
        //}
        //self.history.partial_cmp(&other.history)

        Some(self.cmp(other))
    }
}

fn shortest_path(g: &Graph, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {
    // create priority queue
    let mut pq = BinaryHeap::new();
    pq.push(Step {
        cost: 0,
        position: start,
        history: vec![],
    });

    // collect all the distances between all the nodes
    // MAX distance by default for nodes we have not traversed
    let mut distances: HashMap<Node, Cost> = g
        .nodes
        .iter()
        .map(|&x| if x == start { (x, 0) } else { (x, usize::MAX) })
        .collect();

    while let Some(Step {
        cost,
        position,
        mut history,
    }) = pq.pop()
    {
        if position == goal {
            history.push(goal);
            return Some((history, cost));
        }

        if let Some(neighbours) = &g.edges.get(&position) {
            for &(next_neighbour, next_cost) in neighbours.iter() {
                if next_cost < distances[&next_neighbour] {
                    // we have found our next step
                    let mut next_step = Step {
                        position: next_neighbour,
                        // update the cost of traversal
                        cost: cost + next_cost,
                        // need to clone, then update
                        history: history.clone(),
                    };

                    // update history
                    next_step.history.push(position);
                    // update pq
                    pq.push(next_step);

                    if let Some(old_cost) = distances.get_mut(&next_neighbour) {
                        // dereferencing the pointer to the old cost and assigning/pointing it to next_cost
                        *old_cost = next_cost;
                    }
                }
            }
        }
    }

    // no paths between start and goal
    None
}

fn main() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    if let Some((path, cost)) = shortest_path(&g, 1000, 9000) {
        println!("1000->9000, {:?} {}", path, cost);
    };
}

#[test]
fn large_graph() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    let path = shortest_path(&g, 1000, 9000);
    assert!(path.is_some());
    assert_eq!(path.unwrap().1, 24);
}
