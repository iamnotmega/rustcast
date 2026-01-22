use std::path::Path;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

pub fn open_url(url: &str) {
    #[cfg(target_os = "macos")]
    macos::open_url(url)
}

pub fn open_settings() {
    #[cfg(target_os = "macos")]
    macos::open_settings()
}

pub fn get_img_handle(path: &Path) -> Option<iced::widget::image::Handle>{
    #[cfg(target_os = "macos")]
    return macos::handle_from_icns(path);

    #[cfg(target_os = "windows")]
    return Some(iced::widget::image::Handle::from_path(path));
}