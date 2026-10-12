//! The text of the status line and the details bar.

use crate::gallery::ImageInfo;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

/// From this many images on, the folder summary flags the folder as large.
pub const LARGE_FOLDER_THRESHOLD: usize = 500;

pub fn zoom_status(zoom: f32) -> String {
    format!("{:.0}%", zoom * 100.0)
}

pub fn image_loaded_status(path: &Path, index: usize, total: usize) -> String {
    format!("{} / {} • {}", index + 1, total, image_name_label(path, 36))
}

pub fn unreadable_status(path: &Path) -> String {
    format!("Impossible de lire {}", image_name_label(path, 36))
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

    let folder_label = truncate_middle(&folder_name, 28);

    if total_images >= LARGE_FOLDER_THRESHOLD {
        format!("{} • {} images • dossier dense", folder_label, total_images)
    } else {
        format!("{} • {} images", folder_label, total_images)
    }
}

pub fn image_details_label(path: &Path, image_info: Option<&ImageInfo>) -> Option<String> {
    let metadata = fs::metadata(path).ok();

    let dimensions_label = image_info
        .map(|info| format!("{}×{}", info.width, info.height))
        .unwrap_or_else(|| "dimensions inconnues".to_string());

    let size_label = metadata
        .as_ref()
        .map(|meta| human_size(meta.len()))
        .unwrap_or_else(|| "taille inconnue".to_string());

    let modified_label = metadata
        .and_then(|meta| meta.modified().ok())
        .and_then(system_time_label)
        .unwrap_or_else(|| "date inconnue".to_string());

    Some(format!(
        "{} • {} • {}",
        dimensions_label, size_label, modified_label
    ))
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

fn human_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;

    match bytes as f64 {
        value if value >= MB => format!("{:.1} Mo", value / MB),
        value if value >= KB => format!("{:.0} Ko", value / KB),
        _ => format!("{} o", bytes),
    }
}

fn system_time_label(time: SystemTime) -> Option<String> {
    let elapsed = SystemTime::now().duration_since(time).ok()?;

    let label = if elapsed.as_secs() < 60 {
        "modifié à l’instant".to_string()
    } else if elapsed.as_secs() < 3_600 {
        format!("modifié il y a {} min", elapsed.as_secs() / 60)
    } else if elapsed.as_secs() < 86_400 {
        format!("modifié il y a {} h", elapsed.as_secs() / 3_600)
    } else {
        format!("modifié il y a {} j", elapsed.as_secs() / 86_400)
    };

    Some(label)
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
    fn folder_status_mentions_dense_folder() {
        let label = folder_loaded_status(Path::new("/tmp/gallery"), LARGE_FOLDER_THRESHOLD);

        assert!(label.contains("gallery"));
        assert!(label.contains("dossier dense"));
    }

    #[test]
    fn folder_status_truncates_long_folder_name() {
        let label = folder_loaded_status(
            Path::new("/tmp/un_dossier_avec_un_nom_vraiment_beaucoup_trop_long_pour_le_footer"),
            12,
        );

        assert!(label.contains('…'));
    }

    #[test]
    fn image_name_label_truncates_long_file_names() {
        let label = image_name_label(
            Path::new("/tmp/un_nom_d_image_vraiment_beaucoup_trop_long_pour_le_footer.png"),
            20,
        );

        assert!(label.contains('…'));
    }
}
