use std::io;
use std::sync::Arc;

use iced::executor;
use iced::font::{self, Font};
use iced::widget::image::Handle;
use iced::widget::{button, column, container, text, Column, Image, Row, Scrollable};
use iced::{Application, Command, Element, Length, Settings, Theme};
use iced_aw::{TabBar, TabBarStyles, TabLabel};
use image::{DynamicImage, RgbaImage};
use map::pick_and_load_images;

mod map;

const ICON_FONT: Font = Font::with_name("icons");

pub fn main() -> iced::Result {
    Imaged::run(Settings::default())
}

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    ImageDecode,
    IO(io::ErrorKind),
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

impl Default for Imaged {
    fn default() -> Self {
        Self {
            tab_index: TabId::default(),
            images: None,
            error: None,
        }
    }
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
                Command::perform(pick_and_load_images(), |res| Message::FilesOpened(res))
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

        let image_view = {
            let mut column = Row::new();
            match &self.images {
                Some(images) => {
                    for image in images {
                        let bytes = image.data.clone();
                        let image = Image::new(Handle::from_pixels(
                            bytes.width(),
                            bytes.height(),
                            bytes.to_rgba8().to_vec(),
                        ))
                        .height(Length::Fill)
                        .width(Length::Fill);
                        column = column.push(image);
                    }
                }
                None => {
                    column = column.push(text("No images selected"));
                }
            }

            // Scrollable::new(column)
            column
        };

        let pick_file_btn = button("Open files").on_press(Message::OpenFileDialog);

        let page = container(column![image_view]).height(Length::Fill);

        let error_text = text(
            self.error
                .clone()
                .and_then(|e| Some(format!("{:?}", e)))
                .unwrap_or("No error".to_owned()),
        );
        let content = column![tab_bar, pick_file_btn, page, error_text].spacing(22);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
