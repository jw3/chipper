use std::path::Path;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub enum ImageType {
    Jpeg,
    Png,
}

impl Display for ImageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageType::Jpeg=> f.write_str("jpg"),
            ImageType::Png => f.write_str("png"),
        }
    }
}

impl FromStr for ImageType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "jpg" | "jpeg" => Ok(ImageType::Jpeg),
            "png" => Ok(ImageType::Jpeg),
            f => Err(format!("{} is not a supported image type", f))
        }
    }
}

pub struct Namer {
    base: String,
    name: String,
}

impl Namer {
    pub fn new(input_path: &str, output_dir: Option<String>) -> Self {
        let source_file = Path::new(input_path);
        let output_path = output_dir.unwrap_or("./".into());
        let base_name: String = source_file.file_name().unwrap().to_str().unwrap().into();
        let source_ext = source_file.extension().unwrap().to_str().unwrap();
        let base_name = base_name.strip_suffix(&format!(".{}", source_ext)).unwrap();

        Namer {
            base: output_path,
            name: base_name.into(),
        }
    }

    pub fn get(&self, key: &str, fmt: ImageType) -> String {
        format!("{}/{}-{}.{}", self.base, self.name, key, fmt)
    }
}


#[cfg(test)]
mod tests {
    use crate::{Namer, ImageType};

    #[test]
    fn namer_relative() {
        let namer = Namer::new("foo.tiff", Some("rel".into()));
        let name = namer.get("bar", ImageType::Jpeg);
        assert_eq!("rel/foo-bar.jpg", name);
    }
}
