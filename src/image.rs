use std::path::PathBuf;

use time::OffsetDateTime;

#[derive(Debug)]
pub struct Image {
    path: PathBuf,
    filename: String,
    name: String,
    filename_dt: OffsetDateTime,
}

#[derive(Default)]
pub struct ImageBuilder {
    path: Option<PathBuf>,
    filename: Option<String>,
    name: Option<String>,
    filename_dt: Option<OffsetDateTime>,
}

impl ImageBuilder {
    pub fn new() -> ImageBuilder {
        ImageBuilder {
            path: None,
            filename: None,
            name: None,
            filename_dt: None,
        }
    }

    pub fn path(mut self, path: PathBuf) -> ImageBuilder {
        self.path = Some(path);
        self
    }

    pub fn filename(mut self, filename: String) -> Self {
        self.filename = Some(filename);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn filename_dt(mut self, filename_dt: OffsetDateTime) -> Self {
        self.filename_dt = Some(filename_dt);
        self
    }

    pub fn build(self) -> Result<Image, String> {
        let path = self.path.ok_or("Missing path")?;
        let filename = self.filename.ok_or("Missing filename")?;
        let name = self.name.ok_or("Missing name")?;
        let filename_dt = self.filename_dt.ok_or("Missing filename_dt")?;

        Ok(Image {
            path,
            filename,
            name,
            filename_dt,
        })
    }
}
