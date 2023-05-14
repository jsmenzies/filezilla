use std::path::PathBuf;

#[derive(Debug)]
pub struct Image {
    path: PathBuf,
    filename: String,
    name: String,
}

#[derive(Default)]
pub struct ImageBuilder {
    path: Option<PathBuf>,
    filename: Option<String>,
    name: Option<String>,
}

impl ImageBuilder {
    pub fn new() -> ImageBuilder {
        ImageBuilder {
            path: None,
            filename: None,
            name: None,
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


    pub fn build(self) -> Result<Image, String> {
        let path = self.path.ok_or("Missing path")?;
        let filename = self.filename.ok_or("Missing filename")?;
        let name = self.name.ok_or("Missing name")?;

        Ok(Image {
            path,
            filename,
            name,
        })
    }
}
