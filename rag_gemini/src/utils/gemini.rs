use anyhow::{Context, Result};
use gemini_rs;
use image::{DynamicImage, ImageBuffer, Rgba};
use lopdf::{Dictionary, Document, Object, Stream};
use pdf_extract::extract_text;
use regex::Regex;
use std::collections::HashMap;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::{BufWriter, Write};


pub struct RAGSystem {
    document_store: HashMap<String, String>,
    processed_text_path: Option<String>,
}

impl RAGSystem {
    pub fn new() -> Self {
        Self {
            document_store: HashMap::new(),
            processed_text_path: None,
        }
    }

    pub fn add_document(&mut self, doc_id: &str, content: &str) {
        self.document_store.insert(doc_id.to_string(), content.to_string());
    }

    pub fn add_pdf_document(&mut self, pdf_path: &str, output_path: &str) -> Result<()> {
        let content = extract_text(pdf_path)
            .context(format!("Failed to extract text from PDF: {}", pdf_path))?;

        if content.is_empty() {
            eprintln!("Warning: Extracted content is empty.");
        } else {
            println!("Extracted {} characters from PDF", content.len());
        }

        let cleaned_content = Self::clean_text(&content);

        let mut file = BufWriter::new(File::create(output_path)
            .context(format!("Failed to create output file: {}", output_path))?);
        file.write_all(cleaned_content.as_bytes())
            .context("Failed to write extracted text to file")?;

        println!("Extracted content saved to {}", output_path);

        self.processed_text_path = Some(output_path.to_string());
        self.add_document(pdf_path, ""); // Placeholder reference

        Ok(())
    }

    fn clean_text(content: &str) -> String {
        let re = Regex::new(r"\s+").unwrap(); //regex of multiple spaces
        re.replace_all(content, " ").trim().to_string()
    }


    pub fn retrieve(&self, query: &str) -> Result<String> {
        let text_path = self.processed_text_path.as_ref()
            .context("No processed text file available")?;
        
        let content = read_to_string(text_path)
            .context(format!("Failed to read processed file: {}", text_path))?;

        if content.is_empty() {
            println!("WARNING: Processed file is empty!");
            return Ok(String::new());
        }

        let query_lower = query.to_lowercase();
        let keywords: Vec<&str> = query_lower.split_whitespace().collect();

        println!("Searching for keywords: {:?}", keywords);

        let content_lower = content.to_lowercase();

        for &keyword in &keywords {
            if let Some(position) = content_lower.find(keyword) {
                let start = position.saturating_sub(300);
                let end = (position + keyword.len() + 300).min(content.len());
                return Ok(content[start..end].to_string());
            }
        }

        println!("No relevant content found for query: \"{}\"", query);
        Ok(String::new())
    }


    // pub fn retrieve_images(&self, pdf_path: &str, output_dir: &str) -> Result<()> {
    //     let doc = Document::load(pdf_path).context("Failed to load PDF document")?;

    //     create_dir_all(output_dir).context("Failed to create output directory")?;

    //     for (page_num, page_obj) in doc.get_pages() {
    //         let page_dict = doc.get_object(page_obj).context("Failed to get page dictionary")?;

    //         if let Object::Dictionary(page_dict) = page_dict {
    //             if let Some(Object::Dictionary(resources)) = page_dict.get(b"Resources").ok() {
    //                 if let Some(Object::Dictionary(xobject_dict)) = resources.get(b"XObject") {
    //                     for (name, &xobject_id) in xobject_dict.iter() {
    //                         if let Ok(Object::Stream(stream)) = doc.get_object(xobject_id) {
    //                             if let Some(image) = Self::extract_image(&stream) {
    //                                 let name_str = String::from_utf8_lossy(name);
    //                                 let output_path = format!("{}/page_{}_{}.png", output_dir, page_num, name_str);
    //                                 image.save(&output_path)
    //                                     .context(format!("Failed to save extracted image: {}", output_path))?;
    //                                 println!("Saved image from page {}: {}", page_num, output_path);
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     Ok(())
    // }

    // fn extract_image(stream: &Stream) -> Option<DynamicImage> {
    //     let dict = &stream.dict;
    //     let width = dict.get(b"Width").and_then(Object::as_i64).unwrap_or(0) as u32;
    //     let height = dict.get(b"Height").and_then(Object::as_i64).unwrap_or(0) as u32;
    //     let filter = dict.get(b"Filter").and_then(Object::as_name);
    //     let data = &stream.content;

    //     if width == 0 || height == 0 {
    //         return None;
    //     }

    //     match filter {
    //         Some("DCTDecode") => image::load_from_memory(data).ok(),
    //         Some("FlateDecode") => ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, data.clone())
    //             .map(DynamicImage::ImageRgba8),
    //         _ => None,
    //     }
    // }
}
