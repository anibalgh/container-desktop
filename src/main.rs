use ui::app::ContainerDesktop;
use ui::theme::ThemeManager;

fn main() -> iced::Result {
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
    .window(iced::window::Settings {
        icon,
        ..iced::window::Settings::default()
    })
    .run()
}
