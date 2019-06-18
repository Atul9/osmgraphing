use osmgraphing::graph;
use osmgraphing::dijkstra;
use dijkstra::ShortestPath;
use graph::Graph;
use graph::Read;
use graph::EdgeOffset;

fn main() {
    let mut graph = Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
        node_count: 0,
        edge_count: 0
    };

    graph.read_graph("germany.fmi").expect("error reading file!");
    graph.set_edge_offset();
    //println!("{}", graph);
    let mut dijkstra = dijkstra::init_dijkstra(&graph);
    dijkstra.compute_shortestPath(0, 100000000);
    println!("Distance to Node 4: {}", dijkstra.get_distance(9990));
}

