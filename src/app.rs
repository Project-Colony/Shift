//! The viewer: its state, what each message does, and how it is drawn.

use crate::format::{
    folder_loaded_status, image_details_label, image_loaded_status, image_name_label,
    truncate_middle, unreadable_status, zoom_status,
};
use crate::gallery::{
    ImageInfo, SUPPORTED_EXTENSIONS, collect_images_in_dir, collect_images_in_same_dir,
    read_image_info,
};
use iced::{
    Alignment, ContentFit, Element, Event, Length, Subscription, Task, Theme, event, keyboard,
    mouse,
    widget::{button, column, container, image, row, scrollable, text},
    window,
};
use rfd::FileDialog;
use std::path::{Path, PathBuf};

const MIN_ZOOM: f32 = 0.25;
const MAX_ZOOM: f32 = 6.0;
const ZOOM_STEP: f32 = 0.25;
const VIEWER_BASE_WIDTH: f32 = 960.0;
const VIEWER_BASE_HEIGHT: f32 = 640.0;
const STATUS_PLACEHOLDER: &str = "";

#[derive(Debug)]
pub struct Shift {
    status: String,
    current_path: Option<PathBuf>,
    current_folder: Option<PathBuf>,
    images_in_dir: Vec<PathBuf>,
    current_index: Option<usize>,
    zoom: f32,
    fit_to_view: bool,
    fullscreen: bool,
    image_info: Option<ImageInfo>,
}

impl Default for Shift {
    fn default() -> Self {
        Self {
            status: STATUS_PLACEHOLDER.to_string(),
            current_path: None,
            current_folder: None,
            images_in_dir: Vec::new(),
            current_index: None,
            zoom: 1.0,
            fit_to_view: true,
            fullscreen: false,
            image_info: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenFile,
    OpenFolder,
    PreviousImage,
    NextImage,
    FirstImage,
    LastImage,
    ZoomIn,
    ZoomOut,
    ZoomBy(f32),
    ResetZoom,
    ToggleFit,
    ToggleFullscreen,
    EventOccurred(Event),
    FilePicked(Option<PathBuf>),
    FolderPicked(Option<PathBuf>),
}

pub fn update(state: &mut Shift, message: Message) -> Task<Message> {
    match message {
        Message::EventOccurred(event) => {
            if let Some(action) = keyboard_action(state, &event) {
                return update(state, action);
            }

            if let Some(action) = mouse_action(state, &event) {
                return update(state, action);
            }

            Task::none()
        }
        Message::OpenFile => Task::perform(pick_file(), Message::FilePicked),
        Message::OpenFolder => Task::perform(pick_folder(), Message::FolderPicked),
        Message::PreviousImage => {
            if let Some(index) = state.current_index
                && index > 0
            {
                set_current_image(state, index - 1);
            }
            Task::none()
        }
        Message::NextImage => {
            if let Some(index) = state.current_index
                && index + 1 < state.images_in_dir.len()
            {
                set_current_image(state, index + 1);
            }
            Task::none()
        }
        Message::FirstImage => {
            if !state.images_in_dir.is_empty() {
                set_current_image(state, 0);
            }
            Task::none()
        }
        Message::LastImage => {
            if let Some(last_index) = state.images_in_dir.len().checked_sub(1) {
                set_current_image(state, last_index);
            }
            Task::none()
        }
        Message::ZoomIn => {
            state.fit_to_view = false;
            state.zoom = (state.zoom + ZOOM_STEP).min(MAX_ZOOM);
            state.status = zoom_status(state.zoom);
            Task::none()
        }
        Message::ZoomOut => {
            state.fit_to_view = false;
            state.zoom = (state.zoom - ZOOM_STEP).max(MIN_ZOOM);
            state.status = zoom_status(state.zoom);
            Task::none()
        }
        Message::ZoomBy(delta) => {
            state.fit_to_view = false;
            state.zoom = (state.zoom + delta).clamp(MIN_ZOOM, MAX_ZOOM);
            state.status = zoom_status(state.zoom);
            Task::none()
        }
        Message::ResetZoom => {
            state.fit_to_view = false;
            state.zoom = 1.0;
            state.status = zoom_status(state.zoom);
            Task::none()
        }
        Message::ToggleFit => {
            state.fit_to_view = !state.fit_to_view;
            state.status = if state.fit_to_view {
                "Mode ajusté.".to_string()
            } else {
                zoom_status(state.zoom)
            };
            Task::none()
        }
        Message::ToggleFullscreen => {
            state.fullscreen = !state.fullscreen;
            state.status = if state.fullscreen {
                "Plein écran.".to_string()
            } else {
                current_image_status(state)
            };

            let fullscreen = state.fullscreen;

            window::latest().then(move |window_id| {
                window_id
                    .map(|id| {
                        window::set_mode(
                            id,
                            if fullscreen {
                                window::Mode::Fullscreen
                            } else {
                                window::Mode::Windowed
                            },
                        )
                    })
                    .unwrap_or_else(Task::none)
            })
        }
        Message::FilePicked(Some(path)) => {
            let images = collect_images_in_same_dir(&path);
            let index = images
                .iter()
                .position(|candidate| candidate == &path)
                .unwrap_or(0);

            state.images_in_dir = images;
            state.current_folder = path.parent().map(Path::to_path_buf);
            set_current_image(state, index);
            Task::none()
        }
        Message::FilePicked(None) => {
            if state.current_path.is_none() {
                state.status = "Aucune image ouverte.".to_string();
            } else {
                state.status = "Sélection annulée.".to_string();
            }
            Task::none()
        }
        Message::FolderPicked(Some(folder)) => {
            let images = collect_images_in_dir(&folder);

            if images.is_empty() {
                state.status = format!("Aucune image trouvée dans {}", folder.display());
                state.current_folder = Some(folder);
                state.current_path = None;
                state.current_index = None;
                state.images_in_dir.clear();
                state.image_info = None;
                state.fit_to_view = true;
                state.zoom = 1.0;
            } else {
                let total_images = images.len();
                state.images_in_dir = images;
                state.current_folder = Some(folder.clone());

                if set_current_image(state, 0) {
                    state.status = folder_loaded_status(&folder, total_images);
                }
            }

            Task::none()
        }
        Message::FolderPicked(None) => {
            state.status = if state.current_path.is_some() {
                "Ouverture de dossier annulée.".to_string()
            } else {
                STATUS_PLACEHOLDER.to_string()
            };
            Task::none()
        }
    }
}

pub fn view(state: &Shift) -> Element<'_, Message> {
    let has_image = state.current_path.is_some();
    let show_controls_bar = !state.fullscreen || !has_image;
    let show_meta_bar = has_image && !state.fullscreen;
    let show_footer = footer_visible(state);

    let controls_bar = container(
        row![
            row![
                button("Ouvrir").on_press(Message::OpenFile),
                button("Dossier").on_press(Message::OpenFolder),
            ]
            .spacing(10)
            .wrap(),
            row![
                button("←").on_press_maybe(
                    state
                        .current_index
                        .and_then(|index| (index > 0).then_some(Message::PreviousImage))
                ),
                button("→").on_press_maybe(state.current_index.and_then(|index| {
                    (index + 1 < state.images_in_dir.len()).then_some(Message::NextImage)
                })),
                button(if state.fit_to_view { "100%" } else { "Ajuster" }).on_press_maybe(
                    state.current_path.as_ref().and(Some(if state.fit_to_view {
                        Message::ResetZoom
                    } else {
                        Message::ToggleFit
                    }))
                ),
                button(if state.fullscreen {
                    "Fenêtre"
                } else {
                    "Plein écran"
                })
                .on_press_maybe(
                    state
                        .current_path
                        .as_ref()
                        .map(|_| Message::ToggleFullscreen)
                ),
            ]
            .spacing(10)
            .wrap(),
        ]
        .width(Length::Fill)
        .spacing(18)
        .align_y(Alignment::Center)
        .wrap(),
    )
    .width(Length::Fill)
    .padding([10, 12])
    .style(container::bordered_box);

    let viewer: Element<'_, Message> = match &state.current_path {
        Some(path) => {
            let image_widget = image(path.clone())
                .width(viewer_width(state))
                .height(viewer_height(state))
                .content_fit(if state.fit_to_view {
                    ContentFit::ScaleDown
                } else {
                    ContentFit::Contain
                });

            let framed_viewer: Element<'_, Message> = if state.fit_to_view {
                container(image_widget)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .padding(12)
                    .into()
            } else {
                scrollable(
                    container(image_widget)
                        .padding(12)
                        .width(Length::Shrink)
                        .height(Length::Shrink),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .direction(scrollable::Direction::Both {
                    vertical: scrollable::Scrollbar::default(),
                    horizontal: scrollable::Scrollbar::default(),
                })
                .into()
            };

            container(framed_viewer)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(container::dark)
                .into()
        }
        None => container(
            column![
                text("Shift").size(34),
                text("Un viewer calme, local, rapide.").size(16),
                row![
                    button("Ouvrir").on_press(Message::OpenFile),
                    button("Dossier").on_press(Message::OpenFolder),
                ]
                .spacing(10)
                .wrap(),
            ]
            .spacing(14)
            .align_x(Alignment::Center),
        )
        .width(Length::Fill)
        .height(360)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into(),
    };

    let meta_bar = show_meta_bar.then(|| {
        container(text(viewer_meta_line(state)).size(13))
            .width(Length::Fill)
            .padding([6, 10])
            .style(container::bordered_box)
    });

    let footer = show_footer.then(|| {
        container(text(&state.status).size(13))
            .width(Length::Fill)
            .padding([6, 10])
            .style(container::bordered_box)
    });

    let content = column![
        show_controls_bar.then_some(controls_bar),
        viewer,
        meta_bar,
        footer
    ]
    .spacing(if has_image && !state.fullscreen {
        10
    } else {
        0
    })
    .padding(if has_image && !state.fullscreen {
        14
    } else {
        0
    })
    // An open image gets the whole window so fit mode can use all of it; the
    // cap only keeps the empty start screen compact.
    .max_width(if has_image || state.fullscreen {
        f32::INFINITY
    } else {
        1280.0
    });

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .into()
}

fn footer_visible(state: &Shift) -> bool {
    state.status != STATUS_PLACEHOLDER && !state.fullscreen
}

fn viewer_meta_line(state: &Shift) -> String {
    let position_label = match (state.current_index, state.images_in_dir.is_empty()) {
        (Some(index), false) => format!("{} / {}", index + 1, state.images_in_dir.len()),
        _ => "Aucune image".to_string(),
    };

    let file_label = state
        .current_path
        .as_ref()
        .and_then(|path| path.file_name())
        .map(|name| truncate_middle(&name.to_string_lossy(), 28))
        .unwrap_or_else(|| "Aucun fichier".to_string());

    let details_label = state
        .current_path
        .as_ref()
        .and_then(|path| image_details_label(path, state.image_info.as_ref()))
        .unwrap_or_else(|| "détails indisponibles".to_string());

    format!("{} • {} • {}", position_label, file_label, details_label)
}

fn viewer_width(state: &Shift) -> Length {
    if state.fit_to_view {
        Length::Fill
    } else {
        Length::Fixed(manual_base_width(state) * state.zoom)
    }
}

fn viewer_height(state: &Shift) -> Length {
    if state.fit_to_view {
        Length::Fill
    } else {
        Length::Fixed(manual_base_height(state) * state.zoom)
    }
}

fn manual_base_width(state: &Shift) -> f32 {
    state
        .image_info
        .as_ref()
        .map(|info| info.width as f32)
        .unwrap_or(VIEWER_BASE_WIDTH)
}

fn manual_base_height(state: &Shift) -> f32 {
    state
        .image_info
        .as_ref()
        .map(|info| info.height as f32)
        .unwrap_or(VIEWER_BASE_HEIGHT)
}

pub fn theme(_state: &Shift) -> Theme {
    Theme::TokyoNight
}

pub fn subscription(_state: &Shift) -> Subscription<Message> {
    event::listen().map(Message::EventOccurred)
}

fn keyboard_action(state: &Shift, event: &Event) -> Option<Message> {
    let Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) = event else {
        return None;
    };

    use iced::keyboard::Key;
    use iced::keyboard::key::Named;

    match key.as_ref() {
        Key::Named(Named::ArrowLeft) if state.current_index.is_some() => {
            Some(Message::PreviousImage)
        }
        Key::Named(Named::ArrowRight) if state.current_index.is_some() => Some(Message::NextImage),
        Key::Character(character)
            if matches_char(character, &["p", "["]) && state.current_index.is_some() =>
        {
            Some(Message::PreviousImage)
        }
        Key::Character(character)
            if matches_char(character, &["n", "]"]) && state.current_index.is_some() =>
        {
            Some(Message::NextImage)
        }
        Key::Named(Named::Home) if state.current_index.is_some() => Some(Message::FirstImage),
        Key::Named(Named::End) if state.current_index.is_some() => Some(Message::LastImage),
        Key::Character(character)
            if matches_char(character, &["+", "=", "."]) && state.current_path.is_some() =>
        {
            Some(Message::ZoomIn)
        }
        Key::Character(character)
            if matches_char(character, &["-", ","]) && state.current_path.is_some() =>
        {
            Some(Message::ZoomOut)
        }
        Key::Character(character)
            if matches_char(character, &["0", "f"]) && state.current_path.is_some() =>
        {
            Some(Message::ToggleFit)
        }
        Key::Character(character)
            if matches_char(character, &["1", "r"]) && state.current_path.is_some() =>
        {
            Some(Message::ResetZoom)
        }
        Key::Character(character)
            if matches_char(character, &["m"]) && state.current_path.is_some() =>
        {
            Some(Message::ToggleFullscreen)
        }
        Key::Character(character) if matches_char(character, &["o"]) => Some(Message::OpenFile),
        Key::Character(character) if matches_char(character, &["d"]) => Some(Message::OpenFolder),
        _ => None,
    }
}

fn matches_char(character: &str, candidates: &[&str]) -> bool {
    let lowered = character.to_lowercase();
    candidates.iter().any(|candidate| lowered == *candidate)
}

fn mouse_action(state: &Shift, event: &Event) -> Option<Message> {
    state.current_path.as_ref()?;

    let Event::Mouse(mouse::Event::WheelScrolled { delta }) = event else {
        return None;
    };

    let vertical = match delta {
        mouse::ScrollDelta::Lines { y, .. } => *y,
        mouse::ScrollDelta::Pixels { y, .. } => *y,
    };

    if vertical.abs() < f32::EPSILON {
        return None;
    }

    let step = zoom_step_for_delta(vertical);

    if vertical > 0.0 {
        Some(Message::ZoomBy(step))
    } else {
        Some(Message::ZoomBy(-step))
    }
}

async fn pick_file() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("Images", SUPPORTED_EXTENSIONS)
        .pick_file()
}

async fn pick_folder() -> Option<PathBuf> {
    FileDialog::new().pick_folder()
}

/// Moves to the image at `index` and returns whether it could be read.
///
/// An unreadable image still becomes the current one, so navigation can step
/// past it, but the status keeps the read error.
fn set_current_image(state: &mut Shift, index: usize) -> bool {
    let Some(path) = state.images_in_dir.get(index).cloned() else {
        return false;
    };

    let keep_manual_zoom = state.current_path.is_some() && !state.fit_to_view;
    let current_zoom = state.zoom;

    state.current_path = Some(path.clone());
    state.current_index = Some(index);

    let image_info = read_image_info(&path);

    if image_info.is_none() {
        state.image_info = None;
        state.status = unreadable_status(&path);
        return false;
    }

    state.image_info = image_info;

    if keep_manual_zoom {
        state.fit_to_view = false;
        state.zoom = current_zoom;
    } else {
        state.fit_to_view = true;
        state.zoom = 1.0;
    }

    state.status = image_loaded_status(&path, index, state.images_in_dir.len());
    true
}

fn current_image_status(state: &Shift) -> String {
    match (state.current_path.as_ref(), state.current_index) {
        (Some(path), Some(index)) if !state.images_in_dir.is_empty() => {
            image_loaded_status(path, index, state.images_in_dir.len())
        }
        (Some(path), _) => image_name_label(path, 36),
        (None, _) => STATUS_PLACEHOLDER.to_string(),
    }
}

fn zoom_step_for_delta(vertical_delta: f32) -> f32 {
    if vertical_delta.abs() >= 8.0 {
        ZOOM_STEP * 2.0
    } else if vertical_delta.abs() >= 2.0 {
        ZOOM_STEP * 1.5
    } else {
        ZOOM_STEP
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gallery::Fixture;
    use std::fs;

    fn open_folder(state: &mut Shift, folder: &Path) {
        let _ = update(state, Message::FolderPicked(Some(folder.to_path_buf())));
    }

    #[test]
    fn manual_zoom_is_preserved_when_switching_images() {
        let fixture = Fixture::new("manual-zoom");
        fixture.png("image1.png");
        fixture.png("image2.png");
        let mut state = Shift::default();
        open_folder(&mut state, &fixture.0);

        let _ = update(&mut state, Message::ZoomBy(1.5));
        let _ = update(&mut state, Message::NextImage);

        assert_eq!(state.current_index, Some(1));
        assert_eq!(
            state.image_info,
            Some(ImageInfo {
                width: 4,
                height: 3
            })
        );
        assert!(!state.fit_to_view);
        assert_eq!(state.zoom, 2.5);
        assert_eq!(state.status, "2 / 2 • image2.png");
    }

    #[test]
    fn zoom_step_scales_with_scroll_strength() {
        assert_eq!(zoom_step_for_delta(0.5), ZOOM_STEP);
        assert_eq!(zoom_step_for_delta(3.0), ZOOM_STEP * 1.5);
        assert_eq!(zoom_step_for_delta(10.0), ZOOM_STEP * 2.0);
    }

    #[test]
    fn empty_folder_resets_viewer_state() {
        let fixture = Fixture::new("empty-folder");
        fixture.png("image1.png");
        let empty = fixture.0.join("empty");
        fs::create_dir(&empty).unwrap();
        let mut state = Shift::default();
        open_folder(&mut state, &fixture.0);
        let _ = update(&mut state, Message::ZoomBy(2.0));
        assert!(state.current_path.is_some());

        open_folder(&mut state, &empty);

        assert_eq!(state.current_folder, Some(empty));
        assert!(state.status.starts_with("Aucune image trouvée"));
        assert!(state.current_path.is_none());
        assert!(state.current_index.is_none());
        assert!(state.images_in_dir.is_empty());
        assert!(state.image_info.is_none());
        assert!(state.fit_to_view);
        assert_eq!(state.zoom, 1.0);
    }

    #[test]
    fn cancelling_file_pick_keeps_empty_state_message() {
        let mut state = Shift::default();

        let _ = update(&mut state, Message::FilePicked(None));

        assert_eq!(state.status, "Aucune image ouverte.");
    }

    #[test]
    fn default_status_is_neutral() {
        let state = Shift::default();
        assert_eq!(state.status, STATUS_PLACEHOLDER);
    }

    #[test]
    fn cancelling_folder_pick_keeps_neutral_status_when_empty() {
        let mut state = Shift::default();

        let _ = update(&mut state, Message::FolderPicked(None));

        assert_eq!(state.status, STATUS_PLACEHOLDER);
    }

    #[test]
    fn neutral_status_keeps_footer_hidden() {
        let mut state = Shift::default();
        assert!(!footer_visible(&state));

        let _ = update(&mut state, Message::FolderPicked(None));
        assert!(!footer_visible(&state));

        let _ = update(&mut state, Message::FilePicked(None));
        assert!(footer_visible(&state));
    }

    #[test]
    fn unreadable_image_status_is_not_overwritten_by_folder_status() {
        let fixture = Fixture::new("unreadable-first");
        fixture.garbage("cassée.png");
        fixture.png("image2.png");
        let mut state = Shift::default();

        open_folder(&mut state, &fixture.0);

        assert_eq!(state.status, "Impossible de lire cassée.png");
        assert_eq!(state.current_index, Some(0));
        assert!(state.image_info.is_none());
    }

    #[test]
    fn unreadable_first_image_does_not_block_navigation() {
        let fixture = Fixture::new("unreadable-navigation");
        fixture.garbage("cassée.png");
        fixture.png("image2.png");
        let mut state = Shift::default();
        open_folder(&mut state, &fixture.0);

        let _ = update(&mut state, Message::NextImage);

        assert_eq!(state.current_index, Some(1));
        assert_eq!(state.status, "2 / 2 • image2.png");
    }

    #[test]
    fn readable_folder_shows_folder_summary() {
        let fixture = Fixture::new("readable-folder");
        fixture.png("image1.png");
        fixture.png("image2.png");
        let mut state = Shift::default();

        open_folder(&mut state, &fixture.0);

        assert_eq!(state.current_index, Some(0));
        assert!(state.status.ends_with("• 2 images"), "{}", state.status);
    }

    #[test]
    fn picked_file_opens_at_its_position_in_the_folder() {
        let fixture = Fixture::new("picked-sibling");
        fixture.png("image1.png");
        let picked = fixture.png("image2.png");
        let mut state = Shift::default();

        let _ = update(&mut state, Message::FilePicked(Some(picked.clone())));

        assert_eq!(state.current_path, Some(picked));
        assert_eq!(state.current_index, Some(1));
        assert_eq!(state.images_in_dir.len(), 2);
    }

    #[test]
    fn picked_file_outside_the_folder_scan_opens_itself() {
        let fixture = Fixture::new("picked-unlisted");
        fixture.png("image1.png");
        let picked = fixture.garbage("scan.dat");
        let mut state = Shift::default();

        let _ = update(&mut state, Message::FilePicked(Some(picked.clone())));

        assert_eq!(state.current_path, Some(picked.clone()));
        assert_eq!(state.images_in_dir, vec![picked]);
        assert_eq!(state.status, "Impossible de lire scan.dat");
    }
}
