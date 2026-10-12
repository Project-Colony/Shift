//! French. Same keys as en.rs, written as French rather than translated word
//! for word.

pub(super) const STRINGS: &[(&str, &str)] = &[
    // Control bar and start screen
    ("open", "Ouvrir"),
    ("open_folder", "Dossier"),
    ("previous_image", "Image précédente"),
    ("next_image", "Image suivante"),
    ("fit", "Ajuster"),
    // French puts a narrow no-break space before the percent sign.
    ("zoom_percent", "{percent}\u{202f}%"),
    ("window", "Fenêtre"),
    ("fullscreen", "Plein écran"),
    ("tagline", "Une visionneuse d’images calme, rapide et locale."),
    ("file_filter_images", "Images"),
    // Status line
    ("status_fit", "Ajusté à la fenêtre."),
    ("status_fullscreen", "Plein écran."),
    ("status_no_image_open", "Aucune image ouverte."),
    ("status_open_cancelled", "Sélection annulée."),
    ("status_folder_cancelled", "Ouverture du dossier annulée."),
    ("status_no_images_in", "Aucune image trouvée dans {folder}"),
    ("status_unreadable", "Impossible de lire {name}"),
    ("status_folder_one", "{folder} • 1 image"),
    ("status_folder", "{folder} • {count} images"),
    ("status_folder_large", "{folder} • {count} images • dossier volumineux"),
    // Details bar
    ("unknown_dimensions", "dimensions inconnues"),
    ("unknown_size", "taille inconnue"),
    ("unknown_date", "date inconnue"),
    ("decimal_separator", ","),
    ("size_bytes", "{size} o"),
    ("size_kb", "{size} Ko"),
    ("size_mb", "{size} Mo"),
    ("modified_now", "modifié à l’instant"),
    ("modified_minutes", "modifié il y a {count} min"),
    ("modified_hours", "modifié il y a {count} h"),
    ("modified_days", "modifié il y a {count} j"),
];
