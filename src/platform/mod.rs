use iced::wgpu::rwh::WindowHandle;

#[cfg(target_os = "macos")]
mod macos;

pub fn set_activation_policy_accessory() {
    #[cfg(target_os = "macos")]
    self::macos::set_activation_policy_accessory();
}

pub fn window_config(handle: &WindowHandle) {
    #[cfg(target_os = "macos")]
    self::macos::macos_window_config(handle);
}

pub fn focus_this_app() {
    #[cfg(target_os = "macos")]
    self::macos::focus_this_app();
}

pub fn transform_process_to_ui_element() {
    #[cfg(target_os = "macos")]
    self::macos::transform_process_to_ui_element();
}

/// The kinds of haptic patterns that can be performed
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum HapticPattern {
    Generic,
    Alignment,
    LevelChange,
}

#[cfg(target_os = "macos")]
pub fn perform_haptic(pattern: HapticPattern) -> bool {
    self::macos::perform_haptic(pattern)
}

#[cfg(not(target_os = "macos"))]
pub fn perform_haptic(_pattern: HapticPattern) -> bool {
    false
}
