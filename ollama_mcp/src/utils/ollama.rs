use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::Value; 
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use pdf_extract::extract_text;





pub struct OllamaMCP {
    """
    OllamaMCP is a system for managing and processing documents using the Ollama API.
    It provides functionality to add documents, extract text from PDFs, generate embeddings,
    and retrieve relevant content based on queries.

    Attributes:
    - document_store: A HashMap to store documents with their IDs as keys.
    - processed_text_path: An optional path to the processed text file.
    - OLLAMA_API_URL: The URL of the Ollama API.

    Methods:
    - new: Initializes the OllamaMCP system with an empty document store and API credentials.
    - add_document: Adds a document to the system and generates an embedding for it.
    - add_pdf_document: Adds a PDF document to the system, extracts text, and saves it to a file.
    - retrieve: Retrieves relevant content based on the query using the Ollama API.
    - extract_text_from_pdf: Extracts text from a PDF file using the Ollama API.
    - generate_embedding: Generates an embedding for the given content using the Ollama API.
    - query_ollama: Sends a query to the Ollama API and retrieves the response.
    - clean_text: Cleans the extracted text by removing extra spaces.

    INCOMING FUNCTIONS:
    - convert_pdf_to_images: Converts a PDF file to images and saves them to a specified directory.
    - process_pdf_images: Processes the images extracted from the PDF and saves them to a specified directory.
    - process_pdf: Processes the PDF file by extracting text and converting it to images.
    """
    document_store: HashMap<String, String>,
    processed_text_path: Option<String>,
    ollama_api_url: String,
}

impl OllamaMCP {
    pub fn new(ollama_api_url: &str, ollama_api_key: &str) -> Self {
        """
        Initializes the OllamaMCP system with an empty document store and Local API credentials.

        Input:
        - ollama_api_url: The URL of the Ollama API.
        - ollama_api_key: The API key for authentication.
        Output:
        - Self: An instance of the OllamaMCP system.

        Note: The document store is a HashMap that will hold the documents added to the system.
        """
        Self {
            document_store: HashMap::new(),
            processed_text_path: None,
            ollama_api_url: ollama_api_url.to_string()
        }
    }

    pub fn add_document(&mut self, doc_id: &str, content: &str) {
        """
        Adds a document to the system and generates an embedding for it.
        The document is stored in the document store with the given ID.

        Input:
        - doc_id: Unique identifier for the document.
        - content: The content of the document to be added.
        Output:
        - Result: Ok if successful, Err if there was an error.

        Note: The embedding is generated using the Ollama API.
        """
        let embedding = self.generate_embedding(content).context("Failed to generate embedding through ollama embedding model")?;
        self.document_store.insert(doc_id.to_string(), content.to_string());
        println!("Document added with ID", doc_id);
        Ok(())
    }

    pub fn add_pdf_document(&mut self, pdf_path: &str, output_path: &str) -> Result<()> {
        """
        Adds a PDF document to the system, extracts text, and saves it to a file.
        The extracted text is also added to the document store.

        Input:
        - pdf_path: Path to the PDF file to be processed.
        - output_path: Path to save the extracted text file.
        Output:
        - Result: Ok if successful, Err if there was an error.

        Note: The extracted text is saved to the specified output path.
        """
        let extracted_text = self.extract_text_from_pdf(pdf_path)
            .context(format!("Failed to extract text from PDF: {}", pdf_path))?;

        if extracted_text.is_empty() {
            eprintln!("Warning: Extracted content is empty.");
        } else {
            println!("Extracted {} characters from PDF", extracted_text.len());
        }

        let mut file = BufWriter::new(File::create(output_path)
            .context(format!("Failed to create output file: {}", output_path))?);
        file.write_all(extracted_text.as_bytes())
            .context("Failed to write extracted text to file")?;

        println!("Extracted content saved to {}", output_path);

        self.processed_text_path = Some(output_path.to_string());
        self.add_document(pdf_path, &extracted_text)?;
        Ok(())
    }


    pub fn retrieve(&self, query: &str) -> Result<String> {
        """
        Retrieves relevant content based on the query using the Ollama API.
        If no relevant content is found, it returns an empty string.

        Input:
        - query: The query string to search for relevant content.
        Output:
        - Result: Ok with the relevant content if found, Err if there was an error.

        Note: The query is sent to the Ollama API, and the response is processed.
        """
        let response = self.query_ollama(query)
            .context("Failed to query Ollama for relevant content")?;

        if response.is_empty() {
            println!("No relevant content found for query: \"{}\"", query);
            return Ok(String::new());
        }

        println!("Query result: {}", response);
        Ok(response)
    }

    fn extract_text_from_pdf(&self, pdf_path: &str) -> Result<String> {
        """
        Extracts text from a PDF file using the local function. (pdf_extract::extract_text)

        Input:
        - pdf_path: Path to the PDF file to be processed.
        Output:
        - Result: Ok with the extracted text if successful, Err if there was an error.

        """
        let client = Client::new();
        let response = client
            .post(format!("{}/extract-pdf", self.ollama_api_url))
            .json(&serde_json::json!({ "pdf_path": pdf_path }))
            .send()
            .context("Failed to send request to Ollama API for PDF extraction")?;

        let response_text = response
            .text()
            .context("Failed to read response from Ollama API")?;

        let content: Value = serde_json::from_str(&response_text)
            .context("Failed to parse JSON response from Ollama API")?;

        content["text"]
            .as_str()
            .map(|s| s.to_string())
            .context("No 'text' field in Ollama API response")
    }

    fn generate_embedding(&self, content: &str) -> Result<Vec<f32>> {
        """
        Generates an embedding for the given content using the Ollama API.

        Input:
        - content: The content for which to generate the embedding.
        Output:
        - Result: Ok with the generated embedding as a vector of f32, Err if there was an error.

        Note: The embedding is generated by sending a request to the Ollama API.
        """
        let client = Client::new();
        let response = client
            .post(format!("{}/generate-embedding", self.ollama_api_url))
            .header("Authorization", format!("Bearer {}", self.ollama_api_key))
            .json(&serde_json::json!({ "content": content }))
            .send()
            .context("Failed to send request to Ollama API for embedding generation")?;

        let response_text = response
            .text()
            .context("Failed to read response from Ollama API")?;

        let content: Value = serde_json::from_str(&response_text)
            .context("Failed to parse JSON response from Ollama API")?;

        content["embedding"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_f64().map(|f| f as f32)).collect())
            .context("No 'embedding' field in Ollama API response")
    }

    fn query_ollama(&self, query: &str) -> Result<String> {
        """
        Sends a query to the Ollama API and retrieves the response.

        Input:
        - query: The query string to be sent to the Ollama API.
        Output:
        - Result: Ok with the response from the Ollama API, Err if there was an error.

        Note: The query is sent to the Ollama API, and the response is processed.
        """
        let client = Client::new();
        let response = client
            .post(format!("{}/query", self.ollama_api_url))
            .header("Authorization", format!("Bearer {}", self.ollama_api_key))
            .json(&serde_json::json!({ "query": query }))
            .send()
            .context("Failed to send query to Ollama API")?;

        let response_text = response
            .text()
            .context("Failed to read response from Ollama API")?;

        let content: Value = serde_json::from_str(&response_text)
            .context("Failed to parse JSON response from Ollama API")?;

        content["result"]
            .as_str()
            .map(|s| s.to_string())
            .context("No 'result' field in Ollama API response")
    }
}