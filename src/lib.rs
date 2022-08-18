use input::StaticFileMacroInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod dir;
mod input;
mod mime;

#[proc_macro]
pub fn serve_dir(input: TokenStream) -> TokenStream {
    let minput = parse_macro_input!(input as StaticFileMacroInput);
    let static_dir = minput.directory.value();
    let file_list = dir::generate_file_list(static_dir.into())
        .unwrap_or_else(|e| panic!("The given directory could not be accessed: {:?}", e));
    let path_list = dir::create_paths(minput.path.value().as_str(), &file_list);
    let mut filestrs = Vec::new();
    for file in file_list {
        filestrs.push(file.to_str().unwrap().to_string());
    }
    let ident = minput.app_ident;
    let output = quote! {
        #(
            #ident.at(#path_list).serve_file(#filestrs).unwrap();
        )*
    };
    output.into()
}

#[proc_macro]
pub fn include_dir(input: TokenStream) -> TokenStream {
    let minput = parse_macro_input!(input as StaticFileMacroInput);
    let static_dir = minput.directory.value();
    let file_list = dir::generate_file_list(static_dir.into())
        .unwrap_or_else(|e| panic!("The given directory could not be accessed: {:?}", e));
    let path_list = dir::create_paths(minput.path.value().as_str(), &file_list);
    let mut filestrs = Vec::new();
    let mimes = mime::detect_mimes(&file_list);
    for file in file_list {
        filestrs.push(file.to_str().unwrap().to_string());
    }
    let ident = minput.app_ident;
    let output = quote! {
        #(
            #ident.at(#path_list).get(|_| async { Ok(tide::Response::builder(200).content_type(#mimes).body(include_str!(concat!("../", #filestrs)))) });
        )*
    };
    output.into()
}

#[proc_macro]
pub fn auto_dir(input: TokenStream) -> TokenStream {
    #[cfg(debug_assertions)]
    return serve_dir(input);
    #[cfg(not(debug_assertions))]
    return include_dir(input);
}

#[cfg(test)]
mod tests {
    use crate::dir::{create_paths, generate_file_list};

    #[test]
    fn test_dir_traversal() {
        let list = generate_file_list("static".into()).unwrap();
        println!("{:?}", list);
        let paths = create_paths("/", &list);
        println!("{:?}", paths);
    }
}
