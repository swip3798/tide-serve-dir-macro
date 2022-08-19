#![doc = include_str!("../README.md")]
use input::{StaticFileMacroInput, IncludeFileMacroInput};
use macro_function::macro_fn;
use proc_macro::TokenStream;
use syn::parse_macro_input;

mod dir;
mod input;
mod mime;
mod macro_function;

/// Serves all files in the given directory using the `serve_file` function.
/// Files that are not available during compile time won't be served, files that were available at compile time but not during runtime will cause a panic
#[proc_macro]
pub fn serve_dir(input: TokenStream) -> TokenStream {
    let minput = parse_macro_input!(input as StaticFileMacroInput);
    let minput = IncludeFileMacroInput{ app_ident: minput.app_ident, path: minput.path, directory: minput.directory, max_file_size: None };
    macro_fn(minput, true)
}

/// Serves all files in the given directory by directly integrating them in the application binary using the `include_str!` macro of the standard library.
/// Greatly increases performance but also obviously increases binary file size and memory usage. Addtionally files can't be changed at compile time.
/// Takes an optional 4th parameter to set the maximum file size in bytes before the file is going to be served dynamically instead of being included in the binary.
#[proc_macro]
pub fn include_dir(input: TokenStream) -> TokenStream {
    let minput = parse_macro_input!(input as IncludeFileMacroInput);
    macro_fn(minput, false)
}

/// Selects how it should serve the directory using the build profile.
/// In debug it uses `serve_dir!`, in release it uses `include_dir!`.
/// Takes an optional 4th parameter to set the maximum file size in bytes before the file is going to be served dynamically instead of being included in the binary.
#[proc_macro]
pub fn auto_serve_dir(input: TokenStream) -> TokenStream {
    let minput = parse_macro_input!(input as IncludeFileMacroInput);
    #[cfg(debug_assertions)]
    return macro_fn(minput, true);
    #[cfg(not(debug_assertions))]
    return macro_fn(minput, false);
}