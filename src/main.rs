use iced::executor;
use iced::font::{self, Font};
use iced::widget::{column, container, Text};
use iced::{Application, Command, Element, Length, Settings, Theme};
use iced_aw::{TabBar, TabBarStyles, TabLabel};

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

#[derive(Default)]
struct Imaged {
    tab_index: TabId,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    FontLoaded(Result<(), font::Error>),
    TabSelected(TabId),
}

impl Application for Imaged {
    type Message = Message;
    type Flags = ();
    type Executor = executor::Default;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self::default(),
            font::load(include_bytes!("../fonts/icons.ttf").as_slice()).map(Message::FontLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("Checkbox - Iced")
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::FontLoaded(_) => (),
            Message::TabSelected(value) => self.tab_index = value,
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let tab_bar = TabBar::new(Message::TabSelected)
            .push(TabId::One, TabLabel::Text(String::from("Encrypt")))
            .push(TabId::Two, TabLabel::Text(String::from("Decrypt")))
            .set_active_tab(&self.tab_index)
            .style(TabBarStyles::Dark);

        let etext = Text::new("Encrypt");
        let dtext = Text::new("Decrypt");

        let tab = match self.tab_index {
            TabId::One => container(column![etext]),
            TabId::Two => container(column![dtext]),
        }
        .height(Length::Fill);

        let content = column![tab_bar, tab].spacing(22);

        container(content)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
