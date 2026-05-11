use infrastructure::ConfigManager;
use ui::app::ContainerDesktop;
use ui::theme::ThemeManager;
use ui::typography;

fn main() -> iced::Result {
    // Load font settings synchronously before the async runtime starts.
    let config_manager = ConfigManager::new().expect("config manager");
    let (font_family, font_size) = config_manager.load_font_settings_sync();
    let default_font = typography::resolve_font(&font_family);

    let icon_bytes: &[u8] = if ThemeManager::os_is_dark() {
        include_bytes!("../assets/icons/dark/icon8x8.png")
    } else {
        include_bytes!("../assets/icons/light/icon8x8.png")
    };

    let icon = iced::window::icon::from_file_data(icon_bytes, None).ok();

    iced::application(
        ContainerDesktop::boot,
        ContainerDesktop::update,
        ContainerDesktop::view,
    )
    .theme(ContainerDesktop::theme)
    .title(ContainerDesktop::title)
    .settings(iced::Settings {
        default_font,
        default_text_size: iced::Pixels(font_size as f32),
        ..iced::Settings::default()
    })
    .window(iced::window::Settings {
        icon,
        ..iced::window::Settings::default()
    })
    .run()
}
