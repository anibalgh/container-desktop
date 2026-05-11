use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Background, Element, Length, Padding, Theme};

pub struct Column {
    pub header: String,
    pub width: f32,
}

pub struct DataTableConfig {
    pub columns: Vec<Column>,
    pub row_height: f32,
}

pub fn data_table<'a, Message: Clone + 'a>(
    config: DataTableConfig,
    rows: Vec<Vec<String>>,
    selected_index: Option<usize>,
    on_select: impl Fn(usize) -> Message + 'a,
) -> Element<'a, Message, Theme, iced::Renderer> {
    let header_columns: Vec<Element<'_, Message, Theme, iced::Renderer>> = config
        .columns
        .iter()
        .map(|col| {
            container(text(col.header.clone()).size(11))
                .width(col.width)
                .padding(Padding::new(8.0))
                .into()
        })
        .collect();

    let header =
        container(row(header_columns).align_y(Alignment::Center)).style(container::bordered_box);

    let divider = container(text("")).height(1).style(|theme: &Theme| {
        let p = theme.extended_palette();
        container::Style {
            background: Some(Background::Color(p.background.strong.color)),
            ..Default::default()
        }
    });

    let row_elements: Vec<Element<'_, Message, Theme, iced::Renderer>> = rows
        .into_iter()
        .enumerate()
        .map(|(i, row_data)| {
            let is_selected = selected_index == Some(i);
            let cells: Vec<Element<'_, Message, Theme, iced::Renderer>> = row_data
                .into_iter()
                .enumerate()
                .map(|(j, cell)| {
                    let size = if j == 0 { 12 } else { 11 };
                    container(text(cell).size(size))
                        .width(config.columns.get(j).map(|c| c.width).unwrap_or(100.0))
                        .padding(Padding::new(6.0).left(8.0))
                        .into()
                })
                .collect();

            let row_btn = button(row(cells).align_y(Alignment::Center).height(28.0))
                .width(Length::Fill)
                .on_press(on_select(i));

            if is_selected {
                row_btn.style(selected_row_style).into()
            } else {
                row_btn.style(row_style).into()
            }
        })
        .collect();

    let rows_col = column(row_elements).spacing(1).width(Length::Fill);

    container(column![header, divider, scrollable(rows_col).height(Length::Fill)].spacing(0))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

pub fn simple_table<'a, Message: Clone + 'a>(
    config: DataTableConfig,
    rows: Vec<Vec<String>>,
) -> Element<'a, Message, Theme, iced::Renderer> {
    let header_columns: Vec<Element<'_, Message, Theme, iced::Renderer>> = config
        .columns
        .iter()
        .map(|col| {
            container(text(col.header.clone()).size(11))
                .width(col.width)
                .padding(Padding::new(8.0))
                .into()
        })
        .collect();

    let header =
        container(row(header_columns).align_y(Alignment::Center)).style(container::bordered_box);

    let divider = container(text("")).height(1).style(|theme: &Theme| {
        let p = theme.extended_palette();
        container::Style {
            background: Some(Background::Color(p.background.strong.color)),
            ..Default::default()
        }
    });

    let row_elements: Vec<Element<'_, Message, Theme, iced::Renderer>> = rows
        .into_iter()
        .map(|row_data| {
            let cells: Vec<Element<'_, Message, Theme, iced::Renderer>> = row_data
                .into_iter()
                .enumerate()
                .map(|(j, cell)| {
                    container(text(cell).size(11))
                        .width(config.columns.get(j).map(|c| c.width).unwrap_or(100.0))
                        .padding(Padding::new(6.0).left(8.0))
                        .into()
                })
                .collect();

            container(row(cells).align_y(Alignment::Center).height(24.0))
                .style(|theme: &Theme| {
                    let p = theme.extended_palette();
                    container::Style {
                        background: Some(Background::Color(p.background.base.color)),
                        ..Default::default()
                    }
                })
                .into()
        })
        .collect();

    let rows_col = column(row_elements).spacing(0);

    container(column![header, divider, scrollable(rows_col).height(Length::Fill)].spacing(0))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn row_style(theme: &Theme, status: iced::widget::button::Status) -> iced::widget::button::Style {
    let palette = theme.extended_palette();
    let mut style = iced::widget::button::Style::default();
    match status {
        iced::widget::button::Status::Hovered => {
            style.background = Some(Background::Color(palette.background.strong.color));
        }
        _ => {
            style.background = Some(Background::Color(palette.background.base.color));
        }
    }
    style.text_color = palette.background.base.text;
    style
}

fn selected_row_style(
    theme: &Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    let palette = theme.extended_palette();
    let mut style = iced::widget::button::Style::default();
    match status {
        iced::widget::button::Status::Hovered => {
            style.background = Some(Background::Color(palette.primary.strong.color));
            style.text_color = palette.primary.strong.text;
        }
        _ => {
            style.background = Some(Background::Color(palette.primary.base.color));
            style.text_color = palette.primary.base.text;
        }
    }
    style
}
