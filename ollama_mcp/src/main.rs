use anyhow::Result;
use ollama_mcp::OllamaMCP;

fn main() -> Result<()> {
    // Initialize the OllamaMCP system with API URL and API key
    let ollama_api_url = "https://api.ollama.com"; // Replace with the actual API URL
    let ollama_api_key = "your_api_key_here"; // Replace with your actual API key

    let mut ollama_mcp = OllamaMCP::new(ollama_api_url, ollama_api_key);

    // Example: Adding a document
    let doc_id = "doc1";
    let content = "This is a sample document for testing the OllamaMCP system.";
    ollama_mcp.add_document(doc_id, content)?;

    // Example: Adding a PDF document
    let pdf_path = "path/to/sample.pdf"; // Replace with the actual path to your PDF
    let output_path = "path/to/output.txt"; // Replace with the desired output path for extracted text
    if let Err(e) = ollama_mcp.add_pdf_document(pdf_path, output_path) {
        eprintln!("Error processing PDF document: {}", e);
    }

    
    let query = "What is the content of the sample document?";
    match ollama_mcp.retrieve(query) {
        Ok(response) => {
            if response.is_empty() {
                println!("No relevant content found for query: \"{}\"", query);
            } else {
                println!("Query result: {}", response);
            }
        }
        Err(e) => eprintln!("Error retrieving content: {}", e),
    }

    Ok(())
}