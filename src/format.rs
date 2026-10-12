//! The text of the status line and the details bar, in the active locale.

use crate::gallery::ImageInfo;
use crate::i18n::{self, Locale, fill, t_in, t_with};
use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime};

/// From this many images on, the folder summary flags the folder as large.
pub const LARGE_FOLDER_THRESHOLD: usize = 500;

pub fn zoom_label(zoom: f32) -> String {
    t_with(
        "zoom_percent",
        &[("percent", &format!("{:.0}", zoom * 100.0))],
    )
}

pub fn image_loaded_status(path: &Path, index: usize, total: usize) -> String {
    format!("{} / {} • {}", index + 1, total, image_name_label(path, 36))
}

pub fn unreadable_status(path: &Path) -> String {
    t_with(
        "status_unreadable",
        &[("name", &image_name_label(path, 36))],
    )
}

pub fn no_images_status(folder: &Path) -> String {
    t_with(
        "status_no_images_in",
        &[("folder", &folder.display().to_string())],
    )
}

pub fn image_name_label(path: &Path, max_chars: usize) -> String {
    path.file_name()
        .map(|name| truncate_middle(&name.to_string_lossy(), max_chars))
        .unwrap_or_else(|| path.display().to_string())
}

pub fn folder_loaded_status(folder: &Path, total_images: usize) -> String {
    let folder_name = folder
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| folder.display().to_string());

    let key = match total_images {
        1 => "status_folder_one",
        total if total >= LARGE_FOLDER_THRESHOLD => "status_folder_large",
        _ => "status_folder",
    };

    t_with(
        key,
        &[
            ("folder", &truncate_middle(&folder_name, 28)),
            ("count", &total_images.to_string()),
        ],
    )
}

/// Dimensions, file size and age of the image at `path`.
pub fn image_details_label(path: &Path, image_info: Option<&ImageInfo>) -> String {
    let locale = i18n::locale();
    let metadata = fs::metadata(path).ok();

    let dimensions_label = image_info
        .map(|info| format!("{}×{}", info.width, info.height))
        .unwrap_or_else(|| t_in(locale, "unknown_dimensions").to_string());

    let size_label = metadata
        .as_ref()
        .map(|meta| human_size(meta.len(), locale))
        .unwrap_or_else(|| t_in(locale, "unknown_size").to_string());

    let modified_label = metadata
        .and_then(|meta| meta.modified().ok())
        .and_then(|time| SystemTime::now().duration_since(time).ok())
        .map(|elapsed| modified_label(elapsed, locale))
        .unwrap_or_else(|| t_in(locale, "unknown_date").to_string());

    format!("{dimensions_label} • {size_label} • {modified_label}")
}

pub fn truncate_middle(value: &str, max_chars: usize) -> String {
    let char_count = value.chars().count();

    if char_count <= max_chars {
        return value.to_string();
    }

    if max_chars <= 3 {
        return value.chars().take(max_chars).collect();
    }

    let keep_each_side = (max_chars - 1) / 2;
    let start: String = value.chars().take(keep_each_side).collect();
    let end: String = value
        .chars()
        .rev()
        .take(keep_each_side)
        .collect::<String>()
        .chars()
        .rev()
        .collect();

    format!("{}…{}", start, end)
}

fn human_size(bytes: u64, locale: Locale) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;

    let (key, size) = match bytes as f64 {
        value if value >= MB => ("size_mb", format!("{:.1}", value / MB)),
        value if value >= KB => ("size_kb", format!("{:.0}", value / KB)),
        _ => ("size_bytes", bytes.to_string()),
    };
    let size = size.replace('.', t_in(locale, "decimal_separator"));

    fill(t_in(locale, key), &[("size", &size)])
}

fn modified_label(elapsed: Duration, locale: Locale) -> String {
    let seconds = elapsed.as_secs();
    let (key, count) = match seconds {
        0..60 => return t_in(locale, "modified_now").to_string(),
        60..3_600 => ("modified_minutes", seconds / 60),
        3_600..86_400 => ("modified_hours", seconds / 3_600),
        _ => ("modified_days", seconds / 86_400),
    };

    fill(t_in(locale, key), &[("count", &count.to_string())])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_middle_keeps_short_values_intact() {
        assert_eq!(truncate_middle("eve.png", 16), "eve.png");
    }

    #[test]
    fn truncate_middle_adds_ellipsis_for_long_values() {
        let truncated = truncate_middle("very-long-image-file-name.png", 12);

        assert!(truncated.contains('…'));
        assert!(truncated.chars().count() <= 12);
    }

    #[test]
    fn folder_status_flags_a_large_folder() {
        let label = folder_loaded_status(Path::new("/tmp/gallery"), LARGE_FOLDER_THRESHOLD);

        assert_eq!(label, "gallery • 500 images • large folder");
    }

    #[test]
    fn folder_status_counts_a_single_image() {
        let label = folder_loaded_status(Path::new("/tmp/gallery"), 1);

        assert_eq!(label, "gallery • 1 image");
    }

    #[test]
    fn folder_status_truncates_long_folder_name() {
        let label = folder_loaded_status(
            Path::new("/tmp/a_folder_with_a_name_far_too_long_for_the_status_line"),
            12,
        );

        assert!(label.contains('…'));
    }

    #[test]
    fn image_name_label_truncates_long_file_names() {
        let label = image_name_label(
            Path::new("/tmp/an_image_name_far_too_long_for_the_status_line.png"),
            20,
        );

        assert!(label.contains('…'));
    }

    #[test]
    fn human_size_uses_the_units_of_each_locale() {
        assert_eq!(human_size(512, Locale::En), "512 B");
        assert_eq!(human_size(2048, Locale::En), "2 KB");
        assert_eq!(human_size(1_572_864, Locale::En), "1.5 MB");

        assert_eq!(human_size(512, Locale::Fr), "512 o");
        assert_eq!(human_size(2048, Locale::Fr), "2 Ko");
        assert_eq!(human_size(1_572_864, Locale::Fr), "1,5 Mo");
    }

    #[test]
    fn modified_label_reads_naturally_in_each_locale() {
        let cases = [
            (59, "modified just now", "modifié à l’instant"),
            (60, "modified 1 min ago", "modifié il y a 1 min"),
            (7_200, "modified 2 h ago", "modifié il y a 2 h"),
            (259_200, "modified 3 d ago", "modifié il y a 3 j"),
        ];

        for (seconds, english, french) in cases {
            let elapsed = Duration::from_secs(seconds);
            assert_eq!(modified_label(elapsed, Locale::En), english);
            assert_eq!(modified_label(elapsed, Locale::Fr), french);
        }
    }
}
