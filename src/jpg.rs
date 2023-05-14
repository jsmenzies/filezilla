use std::path::{Path, PathBuf};

use color_eyre::eyre::eyre;
use regex::Regex;
use time::{Date, OffsetDateTime};

use crate::image;

pub fn process(files: &mut Vec<PathBuf>) -> Result<(), color_eyre::Report> {
    println!("processing {} jpg files", files.len());

    for file in files {
        // cut off first 5 characters of the filename
        let filename = file
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
            .ok_or_else(|| eyre!("failed to retrieve photo filename"))?;

        let name = String::from(&filename[5..]);

        let image = image::ImageBuilder::new()
            .name(name.clone())
            .path(file.clone())
            .filename(filename.clone())
            .filename_dt(OffsetDateTime::now_utc())
            .build().unwrap();

        println!("{:?}", image);


        // let date = date_from_filename(file)?;
        // println!("{}: {}", file.display(), date);
    }
    Ok(())
}

fn date_from_filename(photo: &Path) -> Result<OffsetDateTime, color_eyre::Report> {
    let file_name = photo
        .file_name()
        .ok_or_else(|| eyre!("failed to retrieve photo filename"))?;

    // let img_dt = Regex::new(r"^IMG[-_](\d{4})(\d{2})\d{2}[-_](WA)?\d+\.(jpeg|jpg|JPG)$").unwrap();
    // let dt = Regex::new(r"(?x)(?P<year>\d{4})(?P<month>\d{2})(?P<day>\d{2})").unwrap();

    // let captures = self
    //     .date_from_filename_regex
    //     .captures(
    //         file_name
    //             .to_str()
    //             .ok_or_else(|| eyre!("failed to get file name as string"))?,
    //     )
    //     .ok_or_else(|| eyre!("file name doesn't have date format"))?;
    // let year: u16 = match captures.get(1) {
    //     Some(y) => y.as_str().parse().unwrap(),
    //     None => return Err(eyre!("failed to retrieve year from filename")),
    // };
    // let month: u8 = match captures.get(2) {
    //     Some(m) => m.as_str().parse().unwrap(),
    //     None => return Err(eyre!("failed retrieve month from filename")),
    // };
    Ok(OffsetDateTime::now_utc())
}