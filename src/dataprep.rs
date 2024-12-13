use std::error::Error;
use std::fs::File;
use std::path::Path;
use serde::Deserialize;

// Define the Book struct to hold parsed data
#[derive(Debug, Deserialize)]
pub struct Book {
    #[serde(rename = "id")]
    pub book_id: Option<u32>, // optional
    pub title: String,
    pub authors: String,
    pub average_rating: f64,
    pub isbn: String,
    pub isbn13: String,
    pub language_code: Option<String>, // optional
    pub num_pages: Option<u32>, // Optional field for missing values
    pub ratings_count: u32,
    pub text_reviews_count: u32,
    #[serde(rename = "publication_date")]
    pub publication_date: Option<String>, // MM/DD/YYYY format
    pub publisher: String,
}

pub fn parse_csv(file_path: &Path) -> Result<Vec<Book>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut books = Vec::new();

    for result in rdr.deserialize() {
        match result {
            Ok(book) => books.push(book),
            Err(e) => {
                eprintln!("Skipping invalid record: {}", e);
            }
        }
    }

    Ok(books)
}
