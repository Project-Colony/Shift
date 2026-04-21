use iced::{
    widget::{button, column, container, image, row, text},
    Element, Length, Task, Theme,
};
use rfd::FileDialog;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> iced::Result {
    iced::application(ShiftPrivate::default, update, view)
        .theme(theme)
        .run()
}

#[derive(Debug, Default)]
struct ShiftPrivate {
    status: String,
    current_path: Option<PathBuf>,
    images_in_dir: Vec<PathBuf>,
    current_index: Option<usize>,
}

#[derive(Debug, Clone)]
enum Message {
    OpenFile,
    OpenFolder,
    PreviousImage,
    NextImage,
    FilePicked(Option<PathBuf>),
}

fn update(state: &mut ShiftPrivate, message: Message) -> Task<Message> {
    match message {
        Message::OpenFile => Task::perform(pick_file(), Message::FilePicked),
        Message::OpenFolder => {
            state.status = "Ouverture de dossier bientôt.".to_string();
            Task::none()
        }
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
        Message::FilePicked(Some(path)) => {
            let images = collect_images_in_same_dir(&path);
            let index = images.iter().position(|candidate| candidate == &path).unwrap_or(0);

            state.images_in_dir = images;
            set_current_image(state, index);
            Task::none()
        }
        Message::FilePicked(None) => {
            state.status = "Sélection annulée.".to_string();
            Task::none()
        }
    }
}

fn view(state: &ShiftPrivate) -> Element<'_, Message> {
    let header = text("Shift Private").size(36);
    let subtitle = text("Viewer Colony avec navigation locale.").size(18);

    let open_actions = row![
        button("Ouvrir une image").on_press(Message::OpenFile),
        button("Ouvrir un dossier").on_press(Message::OpenFolder),
    ]
    .spacing(12);

    let nav_actions = row![
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
    .spacing(12);

    let viewer: Element<'_, Message> = match &state.current_path {
        Some(path) => image(path.clone())
            .width(Length::Fill)
            .height(Length::FillPortion(2))
            .into(),
        None => container(text("Aucune image ouverte."))
            .width(Length::Fill)
            .height(220)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into(),
    };

    let footer = if let (Some(index), total) = (state.current_index, state.images_in_dir.len()) {
        if total > 0 {
            format!("Image {} / {}", index + 1, total)
        } else {
            state.status.clone()
        }
    } else if state.status.is_empty() {
        "Prototype viewer prêt.".to_string()
    } else {
        state.status.clone()
    };

    let content = column![header, subtitle, open_actions, nav_actions, viewer, text(footer)]
        .spacing(16)
        .padding(24)
        .max_width(960);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .into()
}

fn theme(_state: &ShiftPrivate) -> Theme {
    Theme::TokyoNight
}

async fn pick_file() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("Images", &["png", "jpg", "jpeg", "webp", "gif", "bmp"])
        .pick_file()
}

fn set_current_image(state: &mut ShiftPrivate, index: usize) {
    if let Some(path) = state.images_in_dir.get(index).cloned() {
        state.status = format!("Image chargée : {}", path.display());
        state.current_path = Some(path);
        state.current_index = Some(index);
    }
}

fn collect_images_in_same_dir(path: &Path) -> Vec<PathBuf> {
    let Some(parent) = path.parent() else {
        return vec![path.to_path_buf()];
    };

    let mut images: Vec<PathBuf> = fs::read_dir(parent)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.filter_map(Result::ok))
        .map(|entry| entry.path())
        .filter(|candidate| is_supported_image(candidate))
        .collect();

    images.sort();

    if images.is_empty() {
        vec![path.to_path_buf()]
    } else {
        images
    }
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
