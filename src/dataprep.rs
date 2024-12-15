use std::error::Error;
use std::fs::File;
use std::path::Path;
use serde::Deserialize;

// book struct to hold parsed data
// some attribute are optional since the graphs doesn't directly require it + prevent missing value/ crashing
#[allow(dead_code)] // for testing to get rid of multiple warnings
#[derive(Debug, Deserialize)]
pub struct Book {
    #[serde(rename = "id")] // connect book_id to id
    pub book_id: Option<u32>, // optional 
    pub title: String,
    pub authors: String,
    pub average_rating: f64,
    pub isbn: String,
    pub isbn13: String,
    pub language_code: Option<String>, // optional
    pub num_pages: Option<u32>, // optional field due to missing values (prevent crashing)
    pub ratings_count: u32,
    pub text_reviews_count: u32,
    #[serde(rename = "publication_date")]
    pub publication_date: Option<String>, // MM/DD/YYYY format
    pub publisher: String,
}

// reformating the data defaulting it to a specific data type incase of missing value
impl Default for Book {
    fn default() -> Self {
        Self {
            book_id: None,                           // None 
            title: "".to_string(),                   // Empty string
            authors: "".to_string(),                 // Empty string
            average_rating: 0.0,                     // 0.0
            isbn: "".to_string(),                    // Empty string
            isbn13: "".to_string(),                  // Empty string
            language_code: None,                     // None 
            num_pages: None,                         // None
            ratings_count: 0,                        // 0
            text_reviews_count: 0,                   // 0
            publication_date: None,                  // None
            publisher: "".to_string(),               // Empty string
        }
    }
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

