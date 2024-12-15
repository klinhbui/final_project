use petgraph::graph::DiGraph;
use petgraph::visit::EdgeRef;
use std::collections::HashMap;
use crate::dataprep::Book;

pub fn build_graph(books: &[Book]) -> DiGraph<&Book, f64> {
    let mut graph = DiGraph::new();
    let mut node_map = HashMap::new();

    // Add nodes = book
    for book in books {
        let node = graph.add_node(book);
        node_map.insert(book.title.clone(), node); // Use title as the key
    }

    // Add edges based on relationships: Shared Author(0.1); Similar Rating (1.0); Pages (0.5); Shared Publisher (0.2)
    for book in books {
        if let Some(&source) = node_map.get(&book.title) {
            for (target_title, &target) in &node_map {
                if &book.title != target_title {
                    let target_book = graph.node_weight(target).unwrap().clone();
                    let mut weight = 0.0;

                    // Shared authors
                    if book.authors == target_book.authors {
                        weight += 0.1;
                    }

                    // Similar average ratings
                    if (book.average_rating - target_book.average_rating).abs() <= 0.5 {
                        weight += 1.0;
                    }

                    // Similar number of pages
                    if let (Some(pages1), Some(pages2)) = (book.num_pages, target_book.num_pages) {
                        if (pages1 as i32 - pages2 as i32).abs() <= 50 {
                            weight += 0.5;
                        }
                    }

                    // Same publisher
                    if book.publisher == target_book.publisher {
                        weight += 0.2;
                    }

                    // Add edge if weight > 0
                    if weight > 0.0 {
                        graph.add_edge(source, target, weight);
                    }
                }
            }
        }
    }

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

    // Sort and print the top 10 degrees
    let mut sorted_degrees: Vec<_> = degree_distribution.iter().collect();
    sorted_degrees.sort_by(|a, b| b.0.cmp(a.0)); // Sort by degree descending

    println!("Top 10 Degree Distribution:");
    let top_10 = sorted_degrees.into_iter().take(10).collect::<Vec<_>>();
    for (degree, (count, percentage)) in &top_10 {
        println!("Degree: {}, Count: {}, Percentage: {:.2}%", degree, count, percentage);
    }

    // Return only the top 10 as a HashMap
    top_10.into_iter().map(|(&degree, &stats)| (degree, stats)).collect()
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

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::DiGraph;

    fn sample_books() -> Vec<Book> {
        vec![
            Book {
                title: "Book A".to_string(),
                authors: "Author X".to_string(),
                average_rating: 4.5,
                num_pages: Some(300),
                publisher: "Publisher A".to_string(),
                ..Default::default()
            },
            Book {
                title: "Book B".to_string(),
                authors: "Author X".to_string(),
                average_rating: 4.0,
                num_pages: Some(320),
                publisher: "Publisher A".to_string(),
                ..Default::default()
            },
            Book {
                title: "Book C".to_string(),
                authors: "Author Y".to_string(),
                average_rating: 3.8,
                num_pages: Some(150),
                publisher: "Publisher B".to_string(),
                ..Default::default()
            },
            Book {
                title: "Book D".to_string(),
                authors: "Author Y".to_string(),
                average_rating: 3.9,
                num_pages: Some(145),
                publisher: "Publisher B".to_string(),
                ..Default::default()
            },
        ]
    }

    #[test]
    fn test_build_graph() {
        let books = sample_books();
        let graph = build_graph(&books);

        assert_eq!(graph.node_count(), 4, "Graph should have 4 nodes.");
        assert!(graph.edge_count() > 0, "Graph should have edges.");
    }

    #[test]
    fn test_find_highly_connected_nodes() {
        let books = sample_books();
        let graph = build_graph(&books);

        let top_nodes = find_highly_connected_nodes(&graph);
        assert!(!top_nodes.is_empty(), "There should be highly connected nodes.");
        assert_eq!(top_nodes.len(), 4.min(5), "The number of top nodes should match the graph size or be 5.");
    }

    #[test]
    fn test_analyze_degree_distribution() {
        let books = sample_books();
        let graph = build_graph(&books);

        let degree_distribution = analyze_degree_distribution(&graph);
        assert!(!degree_distribution.is_empty(), "Degree distribution should not be empty.");
        assert!(degree_distribution.iter().all(|(_, (count, _))| *count > 0), "All degrees should have non-zero counts.");
    }

    #[test]
    fn test_find_most_similar_neighbors() {
        let books = sample_books();
        let graph = build_graph(&books);

        let most_similar = find_most_similar_neighbors(&graph);
        assert!(most_similar.is_some(), "There should be a pair of most similar neighbors.");
    }
}
