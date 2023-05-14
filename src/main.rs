use std::collections::HashMap;
use std::path::{Path, PathBuf};

use color_eyre::eyre::bail;

mod jpg;
mod files;

fn main() -> Result<(), color_eyre::Report> {
    color_eyre::install()?;
    let base_dir = Path::new("/home/jsm/jmenzies-exifer-raw");
    // let base_dir = Path::new("/home/jsm/photos");

    let files = files::load_files_in_dir(base_dir)?;
    println!("{} files found", files.len());

    let mut lookup: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for file in &files {
        let extension = match file.extension() {
            Some(ext) => {
                let extension = ext.to_str().unwrap().to_string().to_lowercase();
                if (extension == "jpg") || (extension == "jpeg") {
                    "jpg".to_string()
                } else {
                    extension
                }
            }
            None => bail!("File: {} has no extension", file.display()),
        };
        match lookup.get_mut(&extension) {
            Some(paths) => paths.push(file.clone()),
            None => {
                lookup.insert(extension, vec![file.clone()]);
            }
        }
    }
    //
    // for (key, value) in &lookup {
    //     println!("{}: {}", key, value.len());
    // }
    //

    Ok(())
}
