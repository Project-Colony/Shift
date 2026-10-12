//! What the user chose in Preferences, where it is kept, and the page itself.

use crate::app::Message;
use crate::i18n::{self, Locale, t, t_with};
use colony_ui::preferences::StandardPreferences;
use colony_ui::theme::Palette;
use colony_ui::widgets::{self, ABOUT_CATEGORY, PreferencesView, STANDARD_CATEGORIES};
use colony_ui::{Typography, paths};
use iced::Element;
use iced::widget::{Column, column, text};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashSet;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// The key of the Language section, Shift's own section in General.
const LANGUAGE_SECTION: &str = "language";

/// Everything Preferences can change. The Colony keys sit at the top level of
/// the file, under the names every Colony program writes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Prefs {
    #[serde(flatten)]
    pub standard: StandardPreferences,
    #[serde(default, deserialize_with = "lenient_language")]
    pub language: Language,
}

impl Prefs {
    /// Puts every preference into effect: theme, accent, both text sizes,
    /// fonts, motion, then the language.
    pub fn apply(&self) {
        self.standard.apply();
        i18n::set_locale(self.language.locale());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    /// The system's language, read at each launch.
    #[default]
    Auto,
    En,
    Fr,
}

impl Language {
    const ALL: [Self; 3] = [Self::Auto, Self::En, Self::Fr];

    pub fn locale(self) -> Locale {
        match self {
            Self::Auto => i18n::system_locale(),
            Self::En => Locale::En,
            Self::Fr => Locale::Fr,
        }
    }

    fn label_key(self) -> &'static str {
        match self {
            Self::Auto => "preferences_language_auto",
            Self::En => "preferences_language_en",
            Self::Fr => "preferences_language_fr",
        }
    }
}

/// A language this build does not know, or a value of the wrong type, falls
/// back to Automatic instead of discarding the whole file, as the Colony keys
/// next to it do.
fn lenient_language<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Language, D::Error> {
    let value = serde_json::Value::deserialize(deserializer)?;
    Ok(serde_json::from_value(value).unwrap_or_default())
}

/// `<config>/Colony/Shift/preferences/preferences.json`, found without
/// creating anything. `None` on a system with no configuration folder.
pub fn file_path() -> Option<PathBuf> {
    paths::locate::config_dir(crate::app::NAME)
        .ok()
        .map(|dir| dir.join("preferences").join("preferences.json"))
}

/// The preferences saved at `path`. A missing file is the defaults. A file
/// that cannot be read or parsed is an error for the caller to report; the
/// file itself is left alone until the user changes something.
pub fn load(path: &Path) -> Result<Prefs, String> {
    match fs::read_to_string(path) {
        Ok(json) => serde_json::from_str(&json).map_err(|error| error.to_string()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Prefs::default()),
        Err(error) => Err(error.to_string()),
    }
}

/// Saves `prefs` to `path` without ever leaving a truncated file: the JSON
/// goes to a temporary file in the same folder, reaches the disk, then
/// replaces the old file in one rename.
pub fn save(path: &Path, prefs: &Prefs) -> io::Result<()> {
    let folder = path
        .parent()
        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))?;
    fs::create_dir_all(folder)?;

    let temporary = path.with_extension("json.tmp");
    let written = (|| {
        let mut file = fs::File::create(&temporary)?;
        file.write_all(&serde_json::to_vec_pretty(prefs)?)?;
        file.sync_all()
    })()
    .and_then(|()| fs::rename(&temporary, path));

    if written.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    written
}

/// The Preferences page: General (with Language), Appearance and
/// Accessibility from the Colony kit, then About.
pub fn page<'a>(
    typo: &Typography,
    prefs: &Prefs,
    expanded: &HashSet<String>,
    category: usize,
    file: Option<&Path>,
) -> Element<'a, Message> {
    let kit = PreferencesView::new(
        *typo,
        &prefs.standard,
        expanded,
        Message::Preference,
        Message::ToggleSection,
    );

    let content: Element<'a, Message> = match category {
        0 => kit
            .general()
            .push(language_section(
                typo,
                prefs.language,
                expanded.contains(LANGUAGE_SECTION),
            ))
            .into(),
        1 => kit.appearance().into(),
        2 => kit.accessibility().into(),
        _ => about(typo, file).into(),
    };

    let labels = STANDARD_CATEGORIES
        .iter()
        .map(|key| t(key))
        .chain([t(ABOUT_CATEGORY)]);

    widgets::preferences_page(
        typo,
        widgets::category_list(typo, labels, category, Message::Category),
        content,
        Message::TogglePreferences,
    )
}

fn language_section<'a>(
    typo: &Typography,
    language: Language,
    expanded: bool,
) -> Element<'a, Message> {
    let options = Language::ALL.map(|choice| (choice, t(choice.label_key()).to_string()));

    widgets::collapsible_section(
        typo,
        t("preferences_section_language"),
        expanded,
        Message::ToggleSection(LANGUAGE_SECTION),
        column![
            widgets::section_description(typo, t("preferences_language_desc")),
            widgets::setting_row(
                typo,
                t("preferences_language"),
                t("preferences_language_auto_desc"),
                widgets::dropdown(typo, options, &language, Message::Language),
            ),
        ]
        .into(),
    )
}

fn about<'a>(typo: &Typography, file: Option<&Path>) -> Column<'a, Message> {
    let line = |value: String| {
        text(value)
            .size(typo.sz(13))
            .font(typo.regular)
            .color(Palette::TEXT_SECONDARY())
    };
    let file_line = match file {
        Some(path) => t_with(
            "preferences_about_file",
            &[("path", &path.display().to_string())],
        ),
        None => t("preferences_about_file_none").to_string(),
    };

    widgets::category_heading(typo, t(ABOUT_CATEGORY), t("preferences_about_desc"))
        .push(line(t_with(
            "preferences_about_version",
            &[("version", env!("CARGO_PKG_VERSION"))],
        )))
        .push(line(t("preferences_about_license").to_string()))
        .push(line(t_with(
            "preferences_about_source",
            &[("url", env!("CARGO_PKG_REPOSITORY"))],
        )))
        .push(line(file_line))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gallery::Fixture;
    use colony_ui::{FontSize, TextSize};

    fn custom() -> Prefs {
        let mut prefs = Prefs {
            language: Language::Fr,
            ..Prefs::default()
        };
        prefs.standard.selected_theme = "nord".to_string();
        prefs.standard.selected_variant = "dark".to_string();
        prefs.standard.selected_accent = Some("violet".to_string());
        prefs.standard.font_size = FontSize::Large;
        prefs.standard.text_size_a11y = TextSize::XLarge;
        prefs.standard.dyslexia_font = true;
        prefs
    }

    #[test]
    fn preferences_round_trip_and_leave_no_temporary_file() {
        let fixture = Fixture::new("prefs-round-trip");
        let path = fixture.0.join("preferences").join("preferences.json");

        save(&path, &custom()).unwrap();

        assert_eq!(load(&path), Ok(custom()));
        let files: Vec<_> = fs::read_dir(path.parent().unwrap())
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect();
        assert_eq!(files, ["preferences.json"]);
    }

    #[test]
    fn saving_again_replaces_the_file() {
        let fixture = Fixture::new("prefs-replace");
        let path = fixture.0.join("preferences.json");

        save(&path, &custom()).unwrap();
        save(&path, &Prefs::default()).unwrap();

        assert_eq!(load(&path), Ok(Prefs::default()));
    }

    #[test]
    fn a_missing_file_is_the_defaults() {
        let fixture = Fixture::new("prefs-missing");

        assert_eq!(
            load(&fixture.0.join("preferences.json")),
            Ok(Prefs::default())
        );
    }

    #[test]
    fn a_corrupt_file_is_an_error_and_is_kept() {
        let fixture = Fixture::new("prefs-corrupt");
        let path = fixture.0.join("preferences.json");
        fs::write(&path, "{ not json").unwrap();

        assert!(load(&path).is_err());
        assert_eq!(fs::read_to_string(&path).unwrap(), "{ not json");
    }

    #[test]
    fn a_bad_value_falls_back_and_keeps_the_rest() {
        let fixture = Fixture::new("prefs-lenient");
        let path = fixture.0.join("preferences.json");
        fs::write(
            &path,
            r#"{"language": 42, "selected_theme": "nord", "font_size": "huge", "unknown": true}"#,
        )
        .unwrap();

        let prefs = load(&path).unwrap();

        assert_eq!(prefs.language, Language::Auto);
        assert_eq!(prefs.standard.selected_theme, "nord");
        assert_eq!(prefs.standard.font_size, FontSize::Default);
    }

    #[test]
    fn the_file_uses_the_colony_names() {
        let json = serde_json::to_value(custom()).unwrap();

        assert_eq!(json["selected_theme"], "nord");
        assert_eq!(json["text_size_a11y"], "xlarge");
        assert_eq!(json["language"], "fr");
    }
}
