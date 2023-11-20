use std::fmt::{Debug, Display};
use std::io;
use std::sync::Arc;

use iced::alignment::{Horizontal, Vertical};
use iced::font::{self, Font};
use iced::widget::image::Handle;
use iced::widget::{button, column, container, text, Image, Row};
use iced::{executor, Alignment};
use iced::{Application, Command, Element, Length, Settings, Theme};
use iced_aw::{TabBar, TabBarStyles, TabLabel};
use image::DynamicImage;
use load::pick_and_load_images;

mod load;
mod manipulate;

const _ICON_FONT: Font = Font::with_name("icons");

pub fn main() -> iced::Result {
    Imaged::run(Settings::default())
}

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    ImageDecode,
    IO(io::ErrorKind),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let literal = match self {
            Error::DialogClosed => "Dialog closed",
            Error::ImageDecode => "Image decode failed",
            _ => "Error",
        };

        write!(f, "Error | {}", literal)
    }
}

#[derive(PartialEq, Hash, Clone, Copy, Debug, Eq, Default)]
enum TabId {
    #[default]
    Encrypt,
    Decrypt,
}

#[derive(Debug)]
struct ImageData {
    image_type: TabId,
    data: DynamicImage,
}

#[derive(Default)]
struct Imaged {
    tab_index: TabId,
    images: Option<Vec<ImageData>>,
    error: Option<Error>,
}

#[derive(Debug, Clone)]
enum Message {
    FontLoaded(Result<(), font::Error>),
    TabSelected(TabId),
    OpenFileDialog,
    FilesOpened(Result<Arc<Vec<DynamicImage>>, Error>),
}

impl Application for Imaged {
    type Message = Message;
    type Flags = ();
    type Executor = executor::Default;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self::default(),
            font::load(include_bytes!("../assets/fonts/icons.ttf").as_slice())
                .map(Message::FontLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("Imaged")
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::FontLoaded(_) => Command::none(),
            Message::TabSelected(value) => {
                self.tab_index = value;
                Command::none()
            }
            Message::OpenFileDialog => {
                Command::perform(pick_and_load_images(), Message::FilesOpened)
            }
            Message::FilesOpened(images_res) => {
                match images_res {
                    Ok(images) => {
                        self.images = Some(
                            images
                                .iter()
                                .map(|image| ImageData {
                                    image_type: self.tab_index,
                                    data: image.clone(),
                                })
                                .collect(),
                        )
                    }
                    Err(e) => self.error = Some(e),
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let tab_bar = TabBar::new(Message::TabSelected)
            .push(TabId::Encrypt, TabLabel::Text(String::from("Encrypt")))
            .push(TabId::Decrypt, TabLabel::Text(String::from("Decrypt")))
            .set_active_tab(&self.tab_index)
            .style(TabBarStyles::Dark);

        let pick_file_btn = button("Open files").on_press(Message::OpenFileDialog);
        let encrypt_btn = button(match self.tab_index {
            TabId::Encrypt => "Encrypt",
            TabId::Decrypt => "Decrypt",
        })
        .on_press_maybe(None);
        let button_bar = Row::new().push(pick_file_btn).push(encrypt_btn).spacing(10);

        let image_view = {
            let mut row = Row::new();
            match &self.images {
                Some(images) => {
                    for image in images {
                        if self.tab_index == image.image_type {
                            let bytes = image.data.clone();
                            let image = Image::new(Handle::from_pixels(
                                bytes.width(),
                                bytes.height(),
                                bytes.to_rgba8().to_vec(),
                            ))
                            .height(Length::Fill)
                            .width(Length::Fill);
                            row = row.push(image);
                        }
                    }
                }
                None => {
                    row = row.push(text("No images selected"));
                }
            }

            // Scrollable::new(row)
            row.align_items(Alignment::Center).spacing(5)
        };
        let page = container(column![image_view])
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center);

        let error_text = container(text(
            self.error
                .clone()
                .map(|e| format!("{}", e))
                .unwrap_or("No error".to_owned()),
        ))
        .padding(10);

        let content = column![tab_bar, button_bar, page, error_text]
            .spacing(22)
            .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
