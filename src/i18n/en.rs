//! English, the canonical strings. A new key starts here, then goes to fr.rs.

pub(super) const STRINGS: &[(&str, &str)] = &[
    // Control bar and start screen
    ("open", "Open"),
    ("open_folder", "Folder"),
    ("previous_image", "Previous image"),
    ("next_image", "Next image"),
    ("fit", "Fit"),
    ("zoom_percent", "{percent}%"),
    ("window", "Window"),
    ("fullscreen", "Fullscreen"),
    ("tagline", "A calm, fast, local image viewer."),
    ("file_filter_images", "Images"),
    // Status line
    ("status_fit", "Fit to window."),
    ("status_fullscreen", "Fullscreen."),
    ("status_no_image_open", "No image open."),
    ("status_open_cancelled", "Selection cancelled."),
    ("status_folder_cancelled", "Folder selection cancelled."),
    ("status_no_images_in", "No images found in {folder}"),
    ("status_unreadable", "Cannot read {name}"),
    ("status_folder_one", "{folder} • 1 image"),
    ("status_folder", "{folder} • {count} images"),
    ("status_folder_large", "{folder} • {count} images • large folder"),
    // Details bar
    ("unknown_dimensions", "unknown dimensions"),
    ("unknown_size", "unknown size"),
    ("unknown_date", "unknown date"),
    ("decimal_separator", "."),
    ("size_bytes", "{size} B"),
    ("size_kb", "{size} KB"),
    ("size_mb", "{size} MB"),
    ("modified_now", "modified just now"),
    ("modified_minutes", "modified {count} min ago"),
    ("modified_hours", "modified {count} h ago"),
    ("modified_days", "modified {count} d ago"),
];
