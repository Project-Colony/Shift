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
    (
        "tagline",
        "Une visionneuse d’images calme, rapide et locale.",
    ),
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
    (
        "status_folder_large",
        "{folder} • {count} images • dossier volumineux",
    ),
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
    // Preferences, beyond the shared Colony labels. A no-break space keeps
    // each colon on the line of the word before it.
    ("preferences_section_language", "Langue"),
    (
        "preferences_language_desc",
        "La langue de l’interface. Un changement s’applique aussitôt.",
    ),
    ("preferences_language", "Langue de l’interface"),
    (
        "preferences_language_auto_desc",
        "Automatique suit la langue du système.",
    ),
    ("preferences_language_auto", "Automatique"),
    ("preferences_language_en", "English"),
    ("preferences_language_fr", "Français"),
    (
        "preferences_about_desc",
        "Une visionneuse d’images rapide, locale et pensée pour le clavier.",
    ),
    ("preferences_about_version", "Version {version}"),
    (
        "preferences_about_license",
        "Licence\u{a0}: GPL-3.0-or-later",
    ),
    ("preferences_about_source", "Code source\u{a0}: {url}"),
    (
        "preferences_about_file",
        "Fichier de préférences\u{a0}: {path}",
    ),
    (
        "preferences_about_file_none",
        "Ce système n’a pas de dossier de configuration\u{a0}: les préférences durent jusqu’à la fermeture de Shift.",
    ),
    (
        "preferences_save_failed",
        "Impossible d’enregistrer les préférences\u{a0}: {error}",
    ),
    (
        "preferences_load_failed",
        "Impossible de lire les préférences enregistrées, les valeurs par défaut sont utilisées\u{a0}: {error}",
    ),
];
