use petgraph::graph::DiGraph;
use petgraph::visit::EdgeRef;
use std::collections::HashMap;
use crate::dataprep::Book;

pub fn build_graph(books: &[Book]) -> DiGraph<&Book, f64> {
    let mut graph = DiGraph::new();
    let mut node_map = HashMap::new();
    let mut edge_added = 0; // Declare edge_added

    // Add nodes
    for book in books {
        let node = graph.add_node(book);
        node_map.insert(book.title.clone(), node); // Use title as the key
    }

    // Add edges based on relationships
    for book in books {
        if let Some(&source) = node_map.get(&book.title) {
            for (target_title, &target) in &node_map {
                if &book.title != target_title {
                    let target_book = graph.node_weight(target).unwrap().clone();
                    let mut weight = 0.0;

                    // Similar average ratings
                    if (book.average_rating - target_book.average_rating).abs() <= 1.0 {
                        weight += 0.5;
                    }

                    // Similar number of pages
                    if let (Some(pages1), Some(pages2)) = (book.num_pages, target_book.num_pages) {
                        if (pages1 as i32 - pages2 as i32).abs() <= 300 {
                            weight += 0.5;
                        }
                    }

                    // Same publisher
                    if book.publisher == target_book.publisher {
                        weight += 1.0;
                    }

                    // Add edge if weight > 0
                    if weight > 0.0 {
                        graph.add_edge(source, target, weight);
                        edge_added += 1; // Increment edge count
                    }
                }
            }
        }
    }

    println!("Edges added: {}", edge_added); // Print edge count
    graph
}





pub fn find_highly_connected_nodes<'a>(graph: &'a DiGraph<&'a Book, f64>) -> Vec<(&'a String, usize)> {
    let mut node_connections = graph.node_indices()
        .map(|node_idx| {
            let book = graph[node_idx];
            let degree = graph.edges(node_idx).count();
            (&book.title, degree)
        })
        .collect::<Vec<(&String, usize)>>();

    node_connections.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by top connection count

    node_connections.into_iter().take(5).collect()
}

pub fn analyze_degree_distribution(graph: &DiGraph<&Book, f64>) -> HashMap<usize, (usize, f64)> {
    let mut degree_counts = HashMap::new();
    let total_nodes = graph.node_count();

    // Calculate degree for each node and tally them
    for node_idx in graph.node_indices() {
        let degree = graph.edges(node_idx).count();
        *degree_counts.entry(degree).or_insert(0) += 1;
    }

    // Create a map for degree, count, and percentage
    let mut degree_distribution = HashMap::new();
    for (degree, count) in degree_counts.iter() {
        let percentage = (*count as f64 / total_nodes as f64) * 100.0;
        degree_distribution.insert(*degree, (*count, percentage));
    }

    degree_distribution
}

pub fn find_most_similar_neighbors<'a>(graph: &'a DiGraph<&'a Book, f64>) -> Option<(&'a String, &'a String, f64)> {
    let mut most_similar = None;
    let mut max_similarity = 0.0;

    for edge in graph.edge_references() {
        if let Some((source, target)) = graph.edge_endpoints(edge.id()) {
            let similarity = edge.weight();
            if *similarity > max_similarity {
                max_similarity = *similarity;
                let source_title = &graph[source].title;
                let target_title = &graph[target].title;
                most_similar = Some((source_title, target_title, *similarity));
            }
        }
    }

    most_similar
}
