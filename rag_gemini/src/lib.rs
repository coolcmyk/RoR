//! # RaG-On-Rust (RoR)
//! 
//! A high-performance Retrieval Augmented Generation system built in Rust.
//! This library provides functionality for processing documents (PDFs),
//! extracting text and images, and enhancing queries with Gemini AI.

mod utils;

// Re-export the RAGSystem for external use
pub use crate::utils::geminiRagSystem::RAGSystem;

/// Process a document and retrieve context-enhanced responses
/// 
/// # Example
/// ```no_run
/// use rag_gemini::process_query;
/// 
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let response = process_query("What is RAG?", "path/to/document.pdf").await?;
///     println!("{}", response);
///     Ok(())
/// }
/// ```
pub async fn process_query(query: &str, pdf_path: &str) -> anyhow::Result<String> {
    use tokio::fs::create_dir_all;
    use anyhow::Context;
    use gemini_rs;
    
    // Ensure directories exist
    let _ = create_dir_all("bin").await;
    let text_output_path = "bin/processed.txt";
    
    // Initialize RAG system
    let mut rag_system = RAGSystem::new();
    
    // Process document
    rag_system.add_pdf_document(pdf_path, text_output_path)
        .context("Failed to process PDF document")?;
    
    // Retrieve context for the query
    let context = rag_system.retrieve(query)?;
    
    // Generate response with Gemini
    let response = if context.is_empty() {
        gemini_rs::chat("gemini-2.0-flash")
            .send_message(query)
            .await?
            .text()
    } else {
        gemini_rs::chat("gemini-2.0-flash")
            .send_message(&format!("Context: {}\n\nQuestion: {}", context, query))
            .await?
            .text()
    };
    
    Ok(response)
}