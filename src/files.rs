use std::path::{Path, PathBuf};

use color_eyre::eyre::bail;
use walkdir::WalkDir;

pub fn load_files_in_dir(base_dir: &Path) -> Result<Vec<PathBuf>, color_eyre::Report> {
    match base_dir.exists() {
        false => bail!("Base directory does not exist"),
        true => println!("Base directory: {}", base_dir.display()),
    };

    Ok(WalkDir::new(base_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect())
}