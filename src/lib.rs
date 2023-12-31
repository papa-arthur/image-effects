use wasm_bindgen::prelude::*;
use base64::{Engine as _, alphabet, engine::{self, general_purpose}};
use image::load_from_memory;
use image::ImageOutputFormat::Png;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String{
    let base64_to_vector = general_purpose::STANDARD.decode(encoded_file).unwrap();
    let mut  img = load_from_memory(&base64_to_vector).unwrap();
    img = img.grayscale();
    let mut buffer = vec![];
    img.write_to(&mut buffer, Png).unwrap();
    let encoded_img = general_purpose::STANDARD.encode(&buffer);
    let data_url = format!(
        "data:image/png;base64,{}", encoded_img 
    );
    data_url
}