use iced::{
    widget::{button, column, container, image, row, text},
    Alignment, Element, Length, Task, Theme,
};
use rfd::FileDialog;
use std::fs;
use std::path::{Path, PathBuf};

const SUPPORTED_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "webp", "gif", "bmp"];
const MIN_ZOOM: f32 = 0.25;
const MAX_ZOOM: f32 = 6.0;
const ZOOM_STEP: f32 = 0.25;

fn main() -> iced::Result {
    iced::application(ShiftPrivate::default, update, view)
        .theme(theme)
        .run()
}

#[derive(Debug)]
struct ShiftPrivate {
    status: String,
    current_path: Option<PathBuf>,
    current_folder: Option<PathBuf>,
    images_in_dir: Vec<PathBuf>,
    current_index: Option<usize>,
    zoom: f32,
    fit_to_view: bool,
}

impl Default for ShiftPrivate {
    fn default() -> Self {
        Self {
            status: "Prototype viewer prêt.".to_string(),
            current_path: None,
            current_folder: None,
            images_in_dir: Vec::new(),
            current_index: None,
            zoom: 1.0,
            fit_to_view: true,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    OpenFile,
    OpenFolder,
    PreviousImage,
    NextImage,
    ZoomIn,
    ZoomOut,
    ToggleFit,
    FilePicked(Option<PathBuf>),
    FolderPicked(Option<PathBuf>),
}

fn update(state: &mut ShiftPrivate, message: Message) -> Task<Message> {
    match message {
        Message::OpenFile => Task::perform(pick_file(), Message::FilePicked),
        Message::OpenFolder => Task::perform(pick_folder(), Message::FolderPicked),
        Message::PreviousImage => {
            if let Some(index) = state.current_index {
                if index > 0 {
                    set_current_image(state, index - 1);
                }
            }
            Task::none()
        }
        Message::NextImage => {
            if let Some(index) = state.current_index {
                if index + 1 < state.images_in_dir.len() {
                    set_current_image(state, index + 1);
                }
            }
            Task::none()
        }
        Message::ZoomIn => {
            state.fit_to_view = false;
            state.zoom = (state.zoom + ZOOM_STEP).min(MAX_ZOOM);
            state.status = format!("Zoom manuel : {:.0}%", state.zoom * 100.0);
            Task::none()
        }
        Message::ZoomOut => {
            state.fit_to_view = false;
            state.zoom = (state.zoom - ZOOM_STEP).max(MIN_ZOOM);
            state.status = format!("Zoom manuel : {:.0}%", state.zoom * 100.0);
            Task::none()
        }
        Message::ToggleFit => {
            state.fit_to_view = !state.fit_to_view;
            state.status = if state.fit_to_view {
                "Mode ajusté à la fenêtre activé.".to_string()
            } else {
                format!("Mode zoom manuel : {:.0}%", state.zoom * 100.0)
            };
            Task::none()
        }
        Message::FilePicked(Some(path)) => {
            let images = collect_images_in_same_dir(&path);
            let index = images.iter().position(|candidate| candidate == &path).unwrap_or(0);

            state.images_in_dir = images;
            state.current_folder = path.parent().map(Path::to_path_buf);
            set_current_image(state, index);
            Task::none()
        }
        Message::FilePicked(None) => {
            state.status = "Sélection annulée.".to_string();
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
            } else {
                state.images_in_dir = images;
                state.current_folder = Some(folder);
                set_current_image(state, 0);
            }

            Task::none()
        }
        Message::FolderPicked(None) => {
            state.status = "Ouverture de dossier annulée.".to_string();
            Task::none()
        }
    }
}

fn view(state: &ShiftPrivate) -> Element<'_, Message> {
    let title = text("Shift Private").size(40);
    let subtitle = text("Viewer d’images rapide, local et élégant.").size(18);

    let top_bar = container(
        column![
            row![
                column![title, subtitle].spacing(6),
                container(viewer_badge(state))
                    .padding([10, 14])
                    .align_right(Length::Shrink)
            ]
            .align_y(Alignment::Center),
            row![
                button("Ouvrir une image").on_press(Message::OpenFile),
                button("Ouvrir un dossier").on_press(Message::OpenFolder),
                button("← Précédente").on_press_maybe(
                    state
                        .current_index
                        .and_then(|index| (index > 0).then_some(Message::PreviousImage))
                ),
                button("Suivante →").on_press_maybe(
                    state.current_index.and_then(|index| {
                        (index + 1 < state.images_in_dir.len()).then_some(Message::NextImage)
                    })
                ),
            ]
            .spacing(12)
            .wrap(),
            row![
                button("− Zoom").on_press_maybe(
                    state.current_path.as_ref().and(Some(Message::ZoomOut))
                ),
                button(if state.fit_to_view {
                    "Ajusté à la fenêtre"
                } else {
                    "Mode manuel"
                })
                .on_press_maybe(state.current_path.as_ref().and(Some(Message::ToggleFit))),
                button("+ Zoom").on_press_maybe(
                    state.current_path.as_ref().and(Some(Message::ZoomIn))
                ),
                text(viewer_meta_line(state)).size(16),
            ]
            .spacing(12)
            .align_y(Alignment::Center)
            .wrap(),
        ]
        .spacing(14),
    )
    .width(Length::Fill)
    .padding(20);

    let viewer: Element<'_, Message> = match &state.current_path {
        Some(path) => {
            let image_widget = image(path.clone()).width(viewer_width(state)).height(viewer_height(state));

            container(image_widget)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .padding(24)
                .into()
        }
        None => container(
            column![
                text("Aucune image ouverte.").size(28),
                text("Ouvre une image ou un dossier pour commencer.").size(16),
            ]
            .spacing(10)
            .align_x(Alignment::Center),
        )
        .width(Length::Fill)
        .height(360)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into(),
    };

    let footer = container(text(&state.status).size(15))
        .width(Length::Fill)
        .padding([10, 14]);

    let content = column![top_bar, viewer, footer]
        .spacing(18)
        .padding(24)
        .max_width(1280);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .into()
}

fn viewer_badge(state: &ShiftPrivate) -> Element<'_, Message> {
    let label = match (&state.current_index, state.images_in_dir.len()) {
        (Some(index), total) if total > 0 => format!("{} / {}", index + 1, total),
        _ => "Prototype".to_string(),
    };

    text(label).size(16).into()
}

fn viewer_meta_line(state: &ShiftPrivate) -> String {
    let zoom_label = if state.fit_to_view {
        "Ajusté".to_string()
    } else {
        format!("Zoom {:.0}%", state.zoom * 100.0)
    };

    match &state.current_folder {
        Some(folder) => format!("{} • {} images", zoom_label, state.images_in_dir.len().max(1)).replace("  ", " ") + &format!(" • {}", folder.display()),
        None => zoom_label,
    }
}

fn viewer_width(state: &ShiftPrivate) -> Length {
    if state.fit_to_view {
        Length::Fill
    } else {
        Length::Fixed(960.0 * state.zoom)
    }
}

fn viewer_height(state: &ShiftPrivate) -> Length {
    if state.fit_to_view {
        Length::Fill
    } else {
        Length::Fixed(640.0 * state.zoom)
    }
}

fn theme(_state: &ShiftPrivate) -> Theme {
    Theme::TokyoNight
}

async fn pick_file() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("Images", SUPPORTED_EXTENSIONS)
        .pick_file()
}

async fn pick_folder() -> Option<PathBuf> {
    FileDialog::new().pick_folder()
}

fn set_current_image(state: &mut ShiftPrivate, index: usize) {
    if let Some(path) = state.images_in_dir.get(index).cloned() {
        state.current_path = Some(path.clone());
        state.current_index = Some(index);
        state.fit_to_view = true;
        state.status = format!("Image chargée : {}", path.display());
    }
}

fn collect_images_in_same_dir(path: &Path) -> Vec<PathBuf> {
    let Some(parent) = path.parent() else {
        return vec![path.to_path_buf()];
    };

    let mut images = collect_images_in_dir(parent);

    if images.is_empty() {
        vec![path.to_path_buf()]
    } else {
        images.sort();
        images
    }
}

fn collect_images_in_dir(path: &Path) -> Vec<PathBuf> {
    let mut images: Vec<PathBuf> = fs::read_dir(path)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.filter_map(Result::ok))
        .map(|entry| entry.path())
        .filter(|candidate| is_supported_image(candidate))
        .collect();

    images.sort();
    images
}

fn is_supported_image(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase())
            .as_deref(),
        Some("png" | "jpg" | "jpeg" | "webp" | "gif" | "bmp")
    )
}
