use proc_macro::TokenStream;
use quote::quote;

use crate::{input::IncludeFileMacroInput, dir, mime::detect_mime};

pub(crate) fn macro_fn(input: IncludeFileMacroInput, always_serve: bool) -> TokenStream {
    let static_dir = input.directory.value();
    let file_list = dir::generate_file_list(static_dir.into())
        .unwrap_or_else(|e| panic!("The given directory could not be accessed: {:?}", e));
    let path_list = dir::create_paths(input.path.value().as_str(), &file_list);
    let ident = input.app_ident;
    let mut output = quote!();
    let max_file_size = input.max_file_size.and_then(|i| i.base10_parse::<u64>().ok()).unwrap_or(0);
    for (file, path) in file_list.iter().zip(path_list.iter()) {
        let filestr = file.to_str().unwrap().to_string();
        let file_size = file.metadata().map(|m| m.len()).unwrap_or(0);
        if !always_serve && max_file_size != 0 && file_size < max_file_size {
            let mime = detect_mime(file);
            output = quote! {
                #output
                #ident.at(#path).get(|_| async { Ok(tide::Response::builder(200).content_type(#mime).body(&include_bytes!(concat!("../", #filestr))[..])) });
            };
        } else {
            output = quote! {
                #output
                #ident.at(#path).serve_file(#filestr).unwrap();
            };
        }
        
    }
    output.into()
}
