use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, NaiveDate, Utc};
use color_eyre::eyre::eyre;
use color_eyre::Report;
use exif::Exif;
use regex::{Captures, Match, Regex};

use crate::image;

pub fn process(files: &mut Vec<PathBuf>) -> Result<(), Report> {
    println!("processing {} jpg files", files.len());

    let mut success = 0;
    let mut failed = 0;

    for file in files {
        let filename = file
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
            .ok_or_else(|| eyre!("failed to retrieve photo filename"))?;

        let name = String::from(&filename[5..]);

        let name_date = date_from_filename(&name);
        let exif_date = datetime_from_exif(file);

        if exif_date.is_err() && name_date.is_ok() {
            // println!("{}: {}", filename, name_date.unwrap());
            failed += 1;
        }
    }
    println!("{} files processed, {} failed", success, failed);

    Ok(())
}

fn datetime_from_exif(photo: &Path) -> Result<DateTime<Utc>, Report> {
    if let Ok(file) = fs::File::open(photo) {
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader)?;
        let datetime_tag = exif
            .get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY)
            .ok_or_else(|| eyre!("exif DateTimeOriginal tag is missing"))?;
        let datetime_tag = exif
            .get_field(exif::Tag::DateTimeDigitized, exif::In::PRIMARY)
            .ok_or_else(|| eyre!("exif DateTimeOriginal tag is missing"))?;
        println!("{}: {}", photo.display(), datetime_tag.display_value().with_unit(&exif));
        // for f in exif.fields() {
        //     println!("{} {} {}",
        //              f.tag, f.ifd_num, f.display_value().with_unit(&exif));
        // }
        return Ok(Utc::now())
    }
    Err(eyre!("failed to open file: {}", photo.display()))
}

fn date_from_filename(name: &String) -> Result<NaiveDate, Report> {
    // let dt = Regex::new(r"^IMG[-_](\d{4})(\d{2})\d{2}[-_](WA)?\d+\.(jpeg|jpg|JPG)$").unwrap();
    // let dt = Regex::new(r"^(?x)(?P<year>\d{4})(?P<month>\d{2})(?P<day>\d{2})").unwrap();
    let dt = Regex::new(r"^(?:IMG-|IMG_|Screenshot_|VideoCapture_)?(?P<year>\d{4})(?P<month>\d{2})(?P<day>\d{2})").unwrap();
    // let dt = Regex::new(r"(?:IMG-)(\d{4})(\d{2})(\d{2})_(\d{6})(\(\d\))?").unwrap();

    let captures: Option<Captures> = dt.captures(name.as_str());

    if let Some(caps) = captures {
        let year = check_date_in_range(caps.get(1), 2000, 2022);
        let month = check_date_in_range(caps.get(2), 1, 12);
        let day = check_date_in_range(caps.get(3), 1, 31);

        return if let (Ok(y), Ok(m), Ok(d)) = (year, month, day) {
            let date = NaiveDate::from_ymd_opt(y, m as u32, d as u32).unwrap();
            Ok(date)
        } else {
            Err(eyre!("ERROR Date format Y/M/D: {:?}", name))
        };
    }
    Err(eyre!("ERROR Date regex fail: {:?}", name))
}

fn check_date_in_range(opt: Option<Match<>>, lower: i32, upper: i32) -> Result<i32, Report> {
    if let Some(value) = opt {
        if let Ok(parsed_val) = value.as_str().parse::<i32>() {
            if lower <= parsed_val && parsed_val <= upper {
                return Ok(parsed_val);
            }
        }
    }
    Err(eyre!("ERROR Date component fail: {:?}", opt))
}
