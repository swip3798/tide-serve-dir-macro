use std::path::PathBuf;

use walkdir::WalkDir;

pub fn generate_file_list(path: PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut paths = Vec::new();
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.file_type().is_dir() {
            continue;
        }
        paths.push(entry.path().to_path_buf());
    }
    Ok(paths)
}

pub fn create_paths(path_prefix: &str, files: &Vec<PathBuf>) -> Vec<String> {
    let mut paths = Vec::with_capacity(files.len());
    for file in files {
        let parent = file.parent();
        let join;
        if let Some(parent) = parent {
            let mut ancs = Vec::new();
            for anc in parent.ancestors() {
                if let Some(name) = anc.file_name().and_then(|n| n.to_str()) {
                    ancs.push(name);
                }
            }
            join = format!("{}/{}", path_prefix.trim_end_matches('/'), ancs.join("/"));
        } else {
            join = path_prefix.to_string();
        }
        paths.push(format!("{}/{}", join, file.file_name().unwrap().to_str().unwrap()));
    }
    paths
}
