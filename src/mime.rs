use std::path::PathBuf;

use proc_macro2::TokenStream;
use quote::{quote};

pub fn detect_mimes(paths: &Vec<PathBuf>) -> Vec<TokenStream> {
    let mut mimes = Vec::new();
    
    for path in paths {
        let mime = match path.extension().and_then(|s| s.to_str()).unwrap_or_default() {
            "html" => quote!(tide::http::mime::HTML),
            "js" => quote!(tide::http::mime::JAVASCRIPT),
            "css" => quote!(tide::http::mime::CSS),
            "txt" => quote!(tide::http::mime::PLAIN),
            "xml" => quote!(tide::http::mime::XML),
            "json" => quote!(tide::http::mime::JSON),
            "svg" => quote!(tide::http::mime::SVG),
            "png" => quote!(tide::http::mime::PNG),
            "jpg" | "jpeg" => quote!(tide::http::mime::JPEG),
            "wasm" => quote!(tide::http::mime::WASM),
            "ico" => quote!(tide::http::mime::ICO),
            _ => quote!(tide::http::mime::ANY)
        };
        mimes.push(mime.into());
    }
    mimes
}