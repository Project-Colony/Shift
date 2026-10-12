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
    (
        "status_folder_large",
        "{folder} • {count} images • large folder",
    ),
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
    // Preferences, beyond the shared Colony labels
    ("preferences_section_language", "Language"),
    (
        "preferences_language_desc",
        "The language of the interface. A change applies at once.",
    ),
    ("preferences_language", "Interface language"),
    (
        "preferences_language_auto_desc",
        "Automatic follows the language of your system.",
    ),
    ("preferences_language_auto", "Automatic"),
    ("preferences_language_en", "English"),
    ("preferences_language_fr", "Français"),
    (
        "preferences_about_desc",
        "A fast, local, keyboard-friendly image viewer.",
    ),
    ("preferences_about_version", "Version {version}"),
    ("preferences_about_license", "License: GPL-3.0-or-later"),
    ("preferences_about_source", "Source code: {url}"),
    ("preferences_about_file", "Preferences file: {path}"),
    (
        "preferences_about_file_none",
        "This system has no configuration folder, so preferences last until Shift closes.",
    ),
    (
        "preferences_save_failed",
        "Cannot save preferences: {error}",
    ),
    (
        "preferences_load_failed",
        "Cannot read the saved preferences, so the defaults are in use: {error}",
    ),
];
