use std::path::Path;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub type Coord = (u32, u32);

pub struct BBox {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl BBox {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        BBox { x, y, w, h }
    }
}

pub enum ImageType {
    Jpeg,
    Png,
}

impl Display for ImageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageType::Jpeg => f.write_str("jpg"),
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

    pub fn make(&self, key: &str, fmt: ImageType) -> String {
        format!("{}/{}-{}.{}", self.base, self.name, key, fmt)
    }
}

pub fn matrix(dim: (u32, u32), sz: u32) -> Vec<BBox> {
    let (w, h) = (dim.0 / sz, dim.1 / sz);
    let (rx, ry) = (dim.0 % sz, dim.1 % sz);

    let (xc, yc) = ((0..w), 0..h);
    let mut xc: Vec<(u32, u32)> = xc.map(|i| (i * sz, sz)).collect();
    let mut yc: Vec<(u32, u32)> = yc.map(|i| (i * sz, sz)).collect();
    if rx > 0 {
        xc.push((xc.len() as u32 * sz, rx));
    }
    if ry > 0 {
        yc.push((yc.len() as u32 * sz, ry));
    }

    xc.iter().flat_map(|&ww| yc.iter().map(|&hh| BBox::new(ww.0, hh.0, ww.1,  hh.1)).collect::<Vec<BBox>>()).collect()
}


#[cfg(test)]
mod tests {
    use crate::{Namer, ImageType};

    #[test]
    fn namer_relative() {
        let namer = Namer::new("foo.tiff", Some("rel".into()));
        let name = namer.make("bar", ImageType::Jpeg);
        assert_eq!("rel/foo-bar.jpg", name);
    }
}
