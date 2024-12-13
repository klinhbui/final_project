mod dataprep; // Import the dataprep module
mod graph; // Import the graph module


use std::error::Error;
use std::path::Path;
use crate::dataprep::Book;
use crate::graph::{find_highly_connected_nodes, analyze_degree_distribution, find_most_similar_neighbors, build_graph};

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = Path::new("/opt/app-root/src/final_project/books.csv");

    match crate::dataprep::parse_csv(file_path) {
        Ok(books) => {
            println!("Parsed {} books", books.len());

            // Build the graph
            let graph = build_graph(&books);
            println!("Graph built with {} nodes and {} edges.", graph.node_count(), graph.edge_count());

            // Analyze the graph
            // 1. Find the top 5 highly connected nodes
            let top_nodes = find_highly_connected_nodes(&graph);
            println!("Top 5 highly connected books:");
            for (title, degree) in top_nodes {
                println!("Title: {}, Connections: {}", title, degree);
            }

            // 2. Analyze degree distribution
            let degree_distribution = analyze_degree_distribution(&graph);
            let threshold = 50000; // Adjust threshold as needed
            println!("Books with degree greater than {}:", threshold);
            for node_idx in graph.node_indices() {
                let book = graph[node_idx];
                let degree = graph.edges(node_idx).count();
                if degree > threshold {
                    println!("Title: {}, Degree: {}", book.title, degree);
                }
            }
        

            // 3. Find the most similar neighbors
            if let Some((source, target, similarity)) = find_most_similar_neighbors(&graph) {
                println!("Most similar books are '{}' and '{}' with similarity score {:.2}", source, target, similarity);
            } else {
                println!("No similar neighbors found.");
            }
        }
        Err(e) => {
            println!("Failed to parse the CSV file: {}", e);
        }
    }

    Ok(())
}
