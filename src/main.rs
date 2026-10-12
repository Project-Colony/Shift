#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod app;
mod format;
mod gallery;

use std::ffi::OsStr;

fn main() -> iced::Result {
    // Answered before iced starts, so it works without a display: the release
    // workflow runs `shift --version` on headless runners as a smoke test.
    if wants_version(std::env::args_os().skip(1)) {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    iced::application(app::Shift::default, app::update, app::view)
        .title("Shift")
        .theme(app::theme)
        .subscription(app::subscription)
        .run()
}

/// Whether the first argument after the program name is `--version` or `-V`.
fn wants_version(args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> bool {
    args.into_iter()
        .next()
        .is_some_and(|arg| matches!(arg.as_ref().to_str(), Some("--version" | "-V")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_flag_is_only_read_from_the_first_argument() {
        assert!(wants_version(["--version"]));
        assert!(wants_version(["-V"]));
        assert!(!wants_version([""; 0]));
        assert!(!wants_version(["photo.png"]));
        assert!(!wants_version(["photo.png", "--version"]));
        assert!(!wants_version(["-v"]));
    }
}
