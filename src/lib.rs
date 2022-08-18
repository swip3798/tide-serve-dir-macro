use dir::generate_file_list;
use input::StaticFileMacroInput;
use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse_macro_input};

use crate::dir::create_paths;

mod input;
mod dir;


#[proc_macro]
pub fn serve_dir(input: TokenStream) -> TokenStream {
    let minput = parse_macro_input!(input as StaticFileMacroInput);
    let static_dir = minput.directory.value();
    let file_list = dir::generate_file_list(static_dir.into()).unwrap_or_else(|e| panic!("The given directory could not be accessed: {:?}", e));
    let path_list = dir::create_paths(minput.path.value().as_str(), &file_list);
    todo!()
}

#[proc_macro]
pub fn include_dir(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StaticFileMacroInput);
    
    todo!()
}

// #[test]
// fn test_dir_traversal() {
//     let list = generate_file_list("static".into()).unwrap();
//     println!("{:?}", list);
//     let paths = create_paths("/", &list);
//     println!("{:?}", paths);
// }