//! Finding the images of a folder, in natural order, and reading their size.

use image::image_dimensions;
use std::cmp::Ordering;
use std::fs;
use std::path::{Path, PathBuf};

/// The formats Shift opens. The `image` codecs in Cargo.toml must match.
pub const SUPPORTED_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "webp", "gif", "bmp"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
}

pub fn read_image_info(path: &Path) -> Option<ImageInfo> {
    image_dimensions(path)
        .ok()
        .map(|(width, height)| ImageInfo { width, height })
}

/// Lists the supported images next to `path`, or only `path` itself when it
/// is not one of them (for example a file the folder scan skips).
pub fn collect_images_in_same_dir(path: &Path) -> Vec<PathBuf> {
    let images = path.parent().map(collect_images_in_dir).unwrap_or_default();

    if images.iter().any(|candidate| candidate == path) {
        images
    } else {
        vec![path.to_path_buf()]
    }
}

pub fn collect_images_in_dir(path: &Path) -> Vec<PathBuf> {
    let mut images: Vec<PathBuf> = fs::read_dir(path)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.filter_map(Result::ok))
        .map(|entry| entry.path())
        .filter(|candidate| candidate.is_file() && is_supported_image(candidate))
        .collect();

    images.sort_by(|left, right| compare_image_paths(left, right));
    images
}

pub fn is_supported_image(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| {
            SUPPORTED_EXTENSIONS
                .iter()
                .any(|supported| supported.eq_ignore_ascii_case(ext))
        })
}

fn compare_image_paths(left: &Path, right: &Path) -> Ordering {
    natural_path_key(left).cmp(&natural_path_key(right))
}

fn natural_path_key(path: &Path) -> Vec<NaturalChunk> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(split_natural_chunks)
        .unwrap_or_default()
}

fn split_natural_chunks(value: &str) -> Vec<NaturalChunk> {
    let mut chunks = Vec::new();
    let mut current = String::new();
    let mut current_is_digit = None;

    for character in value.chars() {
        let is_digit = character.is_ascii_digit();

        match current_is_digit {
            Some(previous) if previous == is_digit => current.push(character),
            Some(previous) => {
                chunks.push(NaturalChunk::from_part(&current, previous));
                current.clear();
                current.push(character);
                current_is_digit = Some(is_digit);
            }
            None => {
                current.push(character);
                current_is_digit = Some(is_digit);
            }
        }
    }

    if let Some(is_digit) = current_is_digit {
        chunks.push(NaturalChunk::from_part(&current, is_digit));
    }

    chunks
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum NaturalChunk {
    Text(String),
    Number(u64),
}

impl NaturalChunk {
    fn from_part(value: &str, is_digit: bool) -> Self {
        if is_digit {
            Self::Number(value.parse::<u64>().unwrap_or(0))
        } else {
            Self::Text(value.to_ascii_lowercase())
        }
    }
}

/// A scratch folder under the system temp dir, removed when dropped.
#[cfg(test)]
pub struct Fixture(pub PathBuf);

#[cfg(test)]
impl Fixture {
    pub fn new(name: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("shift-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        Self(dir)
    }

    pub fn png(&self, name: &str) -> PathBuf {
        let path = self.0.join(name);
        image::RgbImage::new(4, 3).save(&path).unwrap();
        path
    }

    pub fn garbage(&self, name: &str) -> PathBuf {
        let path = self.0.join(name);
        fs::write(&path, b"not an image").unwrap();
        path
    }
}

#[cfg(test)]
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The codecs are opt-in features of the `image` crate, so a format missing
    /// from Cargo.toml would list files that then fail to open.
    #[test]
    fn every_supported_extension_decodes() {
        let fixture = Fixture::new("codecs");

        for extension in SUPPORTED_EXTENSIONS {
            let path = fixture.0.join(format!("sample.{extension}"));
            image::RgbImage::new(1, 1)
                .save(&path)
                .unwrap_or_else(|error| panic!("cannot write {extension}: {error}"));

            let decoded = image::open(&path)
                .unwrap_or_else(|error| panic!("cannot decode {extension}: {error}"));
            assert_eq!((decoded.width(), decoded.height()), (1, 1), "{extension}");
        }
    }

    #[test]
    fn natural_sort_orders_numeric_suffixes() {
        let left = PathBuf::from("image2.png");
        let right = PathBuf::from("image10.png");

        assert_eq!(compare_image_paths(&left, &right), Ordering::Less);
    }

    #[test]
    fn supported_extensions_are_case_insensitive() {
        assert!(is_supported_image(Path::new("photo.PNG")));
        assert!(is_supported_image(Path::new("frame.WeBp")));
        assert!(!is_supported_image(Path::new("notes.txt")));
        assert!(!is_supported_image(Path::new("png")));
    }
}
