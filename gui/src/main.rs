use std::fmt::{Debug, Display};
use std::io;
use std::sync::Arc;

use iced::alignment::{Horizontal, Vertical};
use iced::font::{self, Font};
use iced::widget::image::Handle;
use iced::widget::{button, column, container, row, text, Image, Row, Space, Text, TextInput};
use iced::{executor, Alignment};
use iced::{Application, Command, Element, Length, Settings, Theme};
use iced_aw::{SelectionList, SelectionListStyles, Spinner, TabBar, TabBarStyles, TabLabel};
use image::DynamicImage;
use load::pick_and_load_images;
use manipulate::{ArnoldCat, EncMethod, Henon, ImageEncyptor};

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
    Validation,
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

#[derive(Debug, Clone)]
enum Message {
    FontLoaded(Result<(), font::Error>),
    TabSelected(TabId),
    OpenFileDialog,
    FilesOpened(Result<Arc<Vec<DynamicImage>>, Error>),
    EncMethodSelected(usize, EncMethod),
    PwdFieldEdited(String),
    EncryptBtnPressed,
}

#[derive()]
struct Imaged {
    tab_index: TabId,
    images: Option<Vec<ImageData>>,
    res_images: Option<Vec<ImageData>>,
    error: Option<Error>,
    password: String,
    enc_method_state: Option<EncMethod>,
    loading: bool,
    // enc_method_state: State<EncMethod>,
}

impl Default for Imaged {
    fn default() -> Self {
        Self {
            tab_index: TabId::default(),
            images: None,
            res_images: None,
            error: None,
            password: "".to_string(),
            enc_method_state: None,
            loading: false,
            // enc_method_state: State::new(Vec::new()),
        }
    }
}

impl Imaged {
    fn get_enc_variant_items(&self) -> Option<Vec<&ImageData>> {
        self.images
            .as_ref()
            .map(|vec| {
                vec.iter()
                    .filter(|val| val.image_type == self.tab_index)
                    .collect()
            })
            .filter(|val: &Vec<&ImageData>| !val.is_empty())
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
            Message::EncMethodSelected(_, val) => {
                self.enc_method_state = Some(val);
                self.password = "".to_string();
                Command::none()
            }
            Message::PwdFieldEdited(val) => {
                self.password = val.clone();
                self.enc_method_state =
                    self.enc_method_state.as_ref().map(|method| match &method {
                        EncMethod::ArnoldCat(_) => {
                            EncMethod::ArnoldCat(Some(ArnoldCat { key: val }))
                        }
                        EncMethod::Henon(_) => EncMethod::Henon(Some(Henon { key: val })),
                    });
                Command::none()
            }
            Message::EncryptBtnPressed => {
                if !self.password.is_empty() {
                    if let Some(method) = &self.enc_method_state {
                        self.loading = true;

                        let images = self.get_enc_variant_items().clone().unwrap().clone();
                        let res = method
                            .encrypt(images.into_iter().map(|val| val.data.clone()).collect())
                            .into_iter()
                            .map(|res| ImageData {
                                data: res,
                                image_type: self.tab_index,
                            })
                            .collect();
                        self.res_images = Some(res);

                        self.loading = false;
                    }
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

        let button_bar = {
            let pick_file_btn = button("Open files").on_press(Message::OpenFileDialog);
            let encrypt_btn = button(match self.tab_index {
                TabId::Encrypt => "Encrypt",
                TabId::Decrypt => "Decrypt",
            })
            .on_press_maybe(
                if self.enc_method_state.is_none()
                || self.get_enc_variant_items().is_none()  // Ensure images of particular variant present
                || self.password.is_empty()
                {
                    None
                } else {
                    Some(Message::EncryptBtnPressed)
                },
            );
            Row::new().push(pick_file_btn).push(encrypt_btn).spacing(10)
        };

        // let select_enc_method = ComboBox::new(
        //     &self.enc_method_state,
        //     "hi",
        //     Some(&EncMethod::ArnoldCat),
        //     Message::EncMethodSelected,
        // );
        let input_bar = {
            let select_enc_method = SelectionList::new_with(
                &[EncMethod::ArnoldCat(None), EncMethod::Henon(None)],
                Message::EncMethodSelected,
                16_f32,
                6_f32,
                SelectionListStyles::Default,
                None,
                Font {
                    ..Default::default()
                },
            )
            .height(Length::Fixed(30_f32));
            let password_input = TextInput::new("Enter password", self.password.as_str())
                .on_input(Message::PwdFieldEdited);
            Row::new()
                .push(select_enc_method)
                .push(password_input)
                .spacing(10)
                .width(500)
        };

        let page = {
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
                row.align_items(Alignment::Center).spacing(5).height(250)
            };
            let res_image_view = {
                let mut row: Row<'_, Message> = Row::new();
                match &self.res_images {
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
                        row = row.push(text("No results yet"));
                    }
                }

                // Scrollable::new(row)
                row.align_items(Alignment::Center).spacing(5).height(250)
            };
            container(column![image_view, res_image_view].align_items(Alignment::Center))
                .height(Length::Fill)
                .width(Length::Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
        };

        let spinner: Element<Message> = if self.loading {
            Spinner::new().into()
        } else {
            // Space::new(Length::Shrink, Length::Fixed(5_f32)).into()
            Text::new(format!("{}", self.loading)).into()
        };

        let error_text = container(text(
            self.error
                .clone()
                .map(|e| format!("{}", e))
                .unwrap_or("No error".to_owned()),
        ))
        .padding(10);

        let content = column![tab_bar, button_bar, input_bar, page, spinner, error_text]
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
