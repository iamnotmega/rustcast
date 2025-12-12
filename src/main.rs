mod app;
mod macos;

use crate::app::Tile;

use global_hotkey::{
    GlobalHotKeyManager,
    hotkey::{Code, HotKey, Modifiers},
};

fn main() -> iced::Result {
    #[cfg(target_os = "macos")]
    {
        macos::set_activation_policy_accessory();
    }

    let manager = GlobalHotKeyManager::new().unwrap();
    let altspace = HotKey::new(Some(Modifiers::ALT), Code::Space);
    manager
        .register_all(&[altspace])
        .expect("Unable to register hotkey");

    iced::daemon(Tile::new, Tile::update, crate::app::view)
        .subscription(Tile::subscription)
        .theme(Tile::theme)
        .run()
}
