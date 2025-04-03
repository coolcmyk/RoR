mod utils;

pub use crate::utils::gemini::RAGSystem;
pub use ic_cdk_macros::query;

//ic query for RAGSystem
#[query]
fn rag_system() -> RAGSystem {
    RAGSystem::new()
}

//ic query for adding a document to RAGSystem
#[query]
fn add_document(doc_id: String, content: String) {
    let mut rag_system = RAGSystem::new();
    rag_system.add_document(&doc_id, &content);
}

//ic query for adding a pdf document to RAGSystem
#[query]
fn add_pdf_document(pdf_path: String, output_path: String) {
    let mut rag_system = RAGSystem::new();
    rag_system.add_pdf_document(&pdf_path, &output_path);
}

//ic query for retrieving a document from RAGSystem

#[query]
fn retrieve(query: String) -> String {
    let rag_system = RAGSystem::new();
    rag_system.retrieve(&query).unwrap()
}

//ic query for cleaning text
#[query]
fn clean_text(content: String) -> String {
    let rag_system = RAGSystem::new();
    rag_system.clean_text(&content)
}

