//! This has all the utility functions that rustcast uses
use std::{path::Path, thread};

use iced::widget::image::Handle;
use icns::IconFamily;
use image::RgbaImage;
use objc2_app_kit::NSWorkspace;
use objc2_foundation::NSURL;

/// This logs an error to the error log file
pub fn icns_data_to_handle(data: Vec<u8>) -> Option<Handle> {
    let family = IconFamily::read(std::io::Cursor::new(&data)).ok()?;

    let icon_type = family.available_icons();

    let icon = family.get_icon_with_type(*icon_type.first()?).ok()?;
    let image = RgbaImage::from_raw(icon.width(), icon.height(), icon.data().to_vec())?;
    Some(Handle::from_rgba(
        image.width(),
        image.height(),
        image.into_raw(),
    ))
}

/// This converts an icns file to an iced image handle
pub(crate) fn handle_from_icns(path: &Path) -> Option<Handle> {
    let data = std::fs::read(path).ok()?;
    icns_data_to_handle(data)
}

/// Open the settings file with the system default editor
pub fn open_settings() {
    thread::spawn(move || {
        NSWorkspace::new().openURL(&NSURL::fileURLWithPath(
            &objc2_foundation::NSString::from_str(&path.to_string_lossy()),
        ));
    }

    #[cfg(target_os = "linux")]
    {
        Command::new(path).status().ok();
    }
}

pub fn index_installed_apps(config: &Config) -> anyhow::Result<Vec<App>> {
    tracing::debug!("Indexing installed apps");
    tracing::debug!("Exclude patterns: {:?}", &config.index_exclude_patterns);
    tracing::debug!("Include patterns: {:?}", &config.index_include_patterns);

    let path = get_config_file_path();
    let config = read_config_file(path.as_path())?;

    if config.index_dirs.is_empty() {
        tracing::debug!("No extra index dirs provided")
    }

    #[cfg(target_os = "windows")]
    {
        use crate::cross_platform::windows::app_finding::get_apps_from_registry;
        use crate::cross_platform::windows::app_finding::index_start_menu;

        let start = Instant::now();

        let mut other_apps = index_start_menu();
        get_apps_from_registry(&mut other_apps);

        let res = config
            .index_dirs
            .par_iter()
            .flat_map(|x| {
                search_dir(
                    &x.path,
                    &config.index_exclude_patterns,
                    &config.index_include_patterns,
                    x.max_depth,
                )
            })
            .chain(other_apps.into_par_iter())
            .collect();

        let end = Instant::now();
        tracing::info!(
            "Finished indexing apps (t = {}s)",
            (end - start).as_secs_f32()
        );

        Ok(res)
    }

    #[cfg(target_os = "macos")]
    {
        let start = Instant::now();

        let res = config
            .index_dirs
            .par_iter()
            .flat_map(|x| {
                search_dir(
                    &x.path,
                    &config.index_exclude_patterns,
                    &config.index_include_patterns,
                    x.max_depth,
                )
            })
            .collect();

        let end = Instant::now();
        tracing::info!(
            "Finished indexing apps (t = {}s)",
            (end - start).as_secs_f32()
        );

        Ok(res)
    }

    #[cfg(target_os = "linux")]
    {
        let start = Instant::now();

        let other_apps = get_installed_linux_apps(&config);

        let start2 = Instant::now();

        let res = config
            .index_dirs
            .par_iter()
            .flat_map(|x| {
                search_dir(
                    &x.path,
                    &config.index_exclude_patterns,
                    &config.index_include_patterns,
                    x.max_depth,
                )
            })
            .chain(other_apps.into_par_iter())
            .collect();

        let end = Instant::now();
        tracing::info!(
            "Finished indexing apps (t = {}s) (t2 = {}s)",
            (end - start).as_secs_f32(),
            (end - start2).as_secs_f32(),
        );

        Ok(res)
    }
}

/// Check if the provided string looks like a valid url
pub fn is_url_like(s: &str) -> bool {
    if s.starts_with("http://") || s.starts_with("https://") {
        return true;
    }
    if !s.contains('.') {
        return false;
    }
    let mut parts = s.split('.');

    let tld = match parts.next_back() {
        Some(p) => p,
        None => return false,
    };

    if tld.is_empty() || tld.len() > 63 || !tld.chars().all(|c| c.is_ascii_alphabetic()) {
        return false;
    }

    parts.all(|label| {
        !label.is_empty()
            && label.len() <= 63
            && label.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
            && !label.starts_with('-')
            && !label.ends_with('-')
    })
}
