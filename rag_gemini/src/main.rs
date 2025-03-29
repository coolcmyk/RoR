use crate::gemini::RAGSystem;
use tokio::main;
use gemini_rs;
use anyhow::Result;



#[tokio::main]
async fn main() -> Result<()> {
    let mut rag_system = RAGSystem::new();
    let pdf_path = "pdfs/rag.pdf";
    let text_output_path = "bin/processed.txt";
    let image_output_dir = "src/DecentraLearn_backend/src/bin";


    if let Err(e) = rag_system.add_pdf_document(pdf_path, text_output_path) {
        eprintln!("Error processing PDF document: {}", e);
    }


    if let Err(e) = rag_system.retrieve_images(pdf_path, image_output_dir) {
        eprintln!("Error converting PDF to images: {}", e);
    }

    // Query processing
    let query = "Siapakah Michael Harditya";
    let context = rag_system.retrieve(query)?;

    println!("Query: {}", query);
    let response = if context.is_empty() {
        println!("No relevant content found in processed text. Proceeding with general knowledge.");
        gemini_rs::chat("gemini-2.0-flash")
            .send_message(query)
            .await?
    } else {
        println!("Retrieved relevant context of {} characters", context.len());
        gemini_rs::chat("gemini-2.0-flash")
            .send_message(&format!("Context: {}\n\nQuestion: {}", context, query))
            .await?
    };

    println!("Response: {}", response);
    Ok(())
}
