use std::sync::Arc;

use iced::executor;
use iced::font::{self, Font};
use iced::widget::{button, column, container, Image};
use iced::{Application, Command, Element, Length, Settings, Theme};
use iced_aw::{TabBar, TabBarStyles, TabLabel};
use map::{pick_and_load_images, Error};

mod map;

const ICON_FONT: Font = Font::with_name("icons");

pub fn main() -> iced::Result {
    Imaged::run(Settings::default())
}

#[derive(PartialEq, Hash, Clone, Copy, Debug, Eq, Default)]
enum TabId {
    #[default]
    One,
    Two,
}

enum ImageType {
    Encode,
    Decode,
}

struct Imaged {
    tab_index: TabId,
    images: Vec<u8>,
    error: Option<Error>,
}

#[derive(Debug, Clone)]
enum Message {
    FontLoaded(Result<(), font::Error>),
    TabSelected(TabId),
    OpenFileDialog,
    FilesOpened(Result<Arc<Vec<u8>>, Error>),
}

impl Default for Imaged {
    fn default() -> Self {
        Self {
            tab_index: TabId::default(),
            images: Vec::new(),
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
            Message::FilesOpened(files) => {
                match files {
                    Ok(f) => self.images = f.to_vec(),
                    Err(e) => self.error = Some(e),
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let tab_bar = TabBar::new(Message::TabSelected)
            .push(TabId::One, TabLabel::Text(String::from("Encrypt")))
            .push(TabId::Two, TabLabel::Text(String::from("Decrypt")))
            .set_active_tab(&self.tab_index)
            .style(TabBarStyles::Dark);

        let image = Image::new("assets/images/image.png")
            .height(Length::Fill)
            .width(Length::Fill);

        let pick_file_btn = button("Open files").on_press(Message::OpenFileDialog);
        let page = container(column![image, pick_file_btn]).height(Length::Fill);

        let content = column![tab_bar, page].spacing(22);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
