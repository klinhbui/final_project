mod dataprep; // Import the dataprep module
use std::error::Error; // Importing the Error trait
use std::path::Path;
use dataprep::parse_csv; // Assuming `parse_csv` is in the dataprep module

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = Path::new("/opt/app-root/src/final_project/books.csv");

    match parse_csv(file_path) {
        Ok(books) => {
            println!("Parsed {} books", books.len());
            for book in books.iter().take(5) {
                let num_pages = book.num_pages.unwrap_or(0); // Default to 0 if missing
                println!("Title: {}, Pages: {}", book.title, num_pages);
            }
        }
        Err(e) => {
            println!("Failed to parse the CSV file: {}", e);
        }
    }

    Ok(())
}
