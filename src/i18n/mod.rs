//! The interface strings, in English and French.
//!
//! English is canonical: a key is named after its English text and is written
//! in `en.rs` first. `fr.rs` holds the same keys, written as French. The tests
//! below fail when the two disagree on a key or on a placeholder.
//!
//! The labels every Colony program shares (the Preferences page skeleton,
//! theme and accent names) live in `colony_ui::i18n` and are not copied here:
//! [`t`] falls back to them. The active locale is colony-ui's, so the shared
//! labels and Shift's own always switch together.

mod en;
mod fr;

pub use colony_ui::i18n::{Locale, locale, set_locale};

/// The string for `key` in the active locale.
pub fn t(key: &str) -> &'static str {
    t_in(locale(), key)
}

/// The string for `key` in `locale`: Shift's own table first, then the shared
/// Colony labels, which fall back to English and then to the key itself.
pub fn t_in(locale: Locale, key: &str) -> &'static str {
    let table = match locale {
        Locale::En => en::STRINGS,
        Locale::Fr => fr::STRINGS,
    };
    table
        .iter()
        .find(|(candidate, _)| *candidate == key)
        .map(|(_, value)| *value)
        .unwrap_or_else(|| colony_ui::i18n::t_in(locale, key))
}

/// The string for `key` in the active locale, its `{name}` placeholders filled.
pub fn t_with(key: &str, vars: &[(&str, &str)]) -> String {
    fill(t(key), vars)
}

/// Replaces each `{name}` in `template` with its value from `vars`, in one
/// pass, so a value that itself contains braces (a folder named `{count}`) is
/// never substituted again. An unknown placeholder is left as it is.
pub fn fill(template: &str, vars: &[(&str, &str)]) -> String {
    let mut filled = String::with_capacity(template.len());
    let mut rest = template;

    while let Some(open) = rest.find('{') {
        filled.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        let value = after.find('}').and_then(|close| {
            vars.iter()
                .find(|(name, _)| *name == &after[..close])
                .map(|(_, value)| (close, *value))
        });

        match value {
            Some((close, value)) => {
                filled.push_str(value);
                rest = &after[close + 1..];
            }
            None => {
                filled.push('{');
                rest = after;
            }
        }
    }

    filled.push_str(rest);
    filled
}

/// The system's language: `LANGUAGE`, `LC_ALL`, `LC_MESSAGES` or `LANG` on
/// Linux, the user's display language on Windows and macOS. English unless it
/// is French.
pub fn system_locale() -> Locale {
    sys_locale::get_locale().map_or(Locale::En, |tag| Locale::from_tag(&tag))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn keys(table: &[(&str, &str)]) -> BTreeSet<String> {
        table.iter().map(|(key, _)| key.to_string()).collect()
    }

    fn placeholders(value: &str) -> BTreeSet<String> {
        value
            .split('{')
            .skip(1)
            .filter_map(|part| part.split_once('}').map(|(name, _)| name.to_string()))
            .collect()
    }

    #[test]
    fn fr_and_en_have_identical_key_sets() {
        let en = keys(en::STRINGS);
        let fr = keys(fr::STRINGS);

        assert_eq!(en.len(), en::STRINGS.len(), "a key is repeated in en.rs");
        assert_eq!(fr.len(), fr::STRINGS.len(), "a key is repeated in fr.rs");
        assert_eq!(
            en.difference(&fr).collect::<Vec<_>>(),
            Vec::<&String>::new(),
            "keys missing from fr.rs"
        );
        assert_eq!(
            fr.difference(&en).collect::<Vec<_>>(),
            Vec::<&String>::new(),
            "keys missing from en.rs"
        );
    }

    #[test]
    fn no_string_is_empty_and_placeholders_match() {
        for (key, english) in en::STRINGS {
            let french = t_in(Locale::Fr, key);
            assert!(!english.trim().is_empty(), "{key} is empty in en.rs");
            assert!(!french.trim().is_empty(), "{key} is empty in fr.rs");
            assert_eq!(
                placeholders(english),
                placeholders(french),
                "{key} has different placeholders in en.rs and fr.rs"
            );
        }
    }

    /// The shared labels are not copied into Shift's tables, yet they must
    /// still resolve through `t_in`, in the requested locale.
    #[test]
    fn shared_labels_come_from_colony_ui() {
        assert_eq!(t_in(Locale::En, "settings_title"), "Preferences");
        assert_eq!(t_in(Locale::Fr, "settings_title"), "Préférences");
        assert!(!keys(en::STRINGS).iter().any(|key| key.starts_with("settings_")));
    }

    #[test]
    fn fill_substitutes_once_and_keeps_unknown_placeholders() {
        assert_eq!(
            fill("{folder} • {count} images", &[("folder", "{count}"), ("count", "3")]),
            "{count} • 3 images"
        );
        assert_eq!(fill("{missing} {", &[]), "{missing} {");
        assert_eq!(fill("no placeholder", &[("a", "b")]), "no placeholder");
    }

}
