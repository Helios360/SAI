use iced::widget::{button, column, container, row, text, svg};
use iced::{Element, Length, Sandbox, Settings, Alignment, application, Color};
use iced::theme::{self, Theme, Custom, Palette};
use iced::border::Radius;
use std::default::Default;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, LazyLock};

// Define the static USERNAME
static USER_NAME: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("default".to_string()));

// Define constant colors
const BACKGROUND: Color = Color::from_rgb(
    40.0 / 255.0,
    47.0 / 255.0,
    55.0 / 255.0
);

const ACCENT: Color = Color::from_rgb(
    0.0,
    0.5,
    1.0
);

// Define the Page enum
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Page {
    Home,Controls,Sensors,Think,Settings,Socials,News,Games,Utils,Music,Infos,Messages
}

// Define the Message enum
#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Page),
    UpdateUserName(String),
    RowClicked(u8),
}

// Define AppData struct
#[derive(Debug, Clone, Default)]
pub struct AppData {
    pub user_name: String,
}

// Define App struct
pub struct App {
    current_page: Page,
    data: AppData,
}

// Define Custom things
struct CustomApplicationStyle;
struct CustomTheme;

impl application::StyleSheet for CustomTheme {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: BACKGROUND,
            text_color: Color::WHITE,
        }
    }
}

impl button::StyleSheet for CustomTheme {
    type Style = Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(ACCENT)),
            border: iced::Border {
                radius: Radius::from(5.0),
                width: 1.0,
                color: Color::BLACK,
            },
            text_color: Color::WHITE,
            ..Default::default()
        }
    }
}

impl CustomTheme {
    fn to_custom(self) -> Arc<Custom> {
        let custom_palette = Palette {
            background: BACKGROUND,
            text: Color::WHITE,
            primary: ACCENT,
            success: Color::from_rgb(0.0, 1.0, 0.0),
            danger: Color::from_rgb(1.0, 0.0, 0.0),
        };
        Arc::new(iced::theme::Custom::new("CustomTheme".to_string(), custom_palette))
    }
}

impl application::StyleSheet for CustomApplicationStyle {
    type Style = Theme;
    fn appearance(&self, _: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: BACKGROUND,
            text_color: Color::WHITE,
        }
    }
}

// Define CustomButtonStyle
#[derive(Debug, Clone, Copy)]
struct CustomButtonStyle {
    is_active: bool,
}

impl button::StyleSheet for CustomButtonStyle {
    type Style = Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(if self.is_active {
                Color::from_rgb(0.98, 0.85, 0.87)
            } else {
                Color::from_rgb(0.2, 0.2, 0.2)
            })),
            border: iced::Border {
                radius: Radius::from(0.0),
                width: 1.0, // Adjust as needed
                color: iced::Color::BLACK, // Adjust as needed
            },
            text_color: Color::WHITE,
            ..Default::default()
        }
    }
}

impl Sandbox for App {
    type Message = Message;
    fn new() -> Self {
        Self {
            current_page: Page::Home,
            data: AppData::default(),
        }
    }
    fn title(&self) -> String {
        String::from("SAI - Dashboard")
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::RowClicked(row) => {
                println!("Row {} was clicked!", row);
                // Add any logic you want when a row is clicked
            }
            Message::Navigate(page) => self.current_page = page,
            Message::UpdateUserName(new_name) => {
                self.data.user_name = new_name;
            },
        }
    }

    fn view(&self) -> Element<Message> {
        // Get the current username from the global state (I hate file var passing)
        let username = USER_NAME.lock().unwrap().clone();
        let menu = column![
            create_nav_button(self, "Home", Page::Home),
            create_nav_button(self, "Controls", Page::Controls),
            create_nav_button(self, "Sensors", Page::Sensors),
            create_nav_button(self, "Think", Page::Think),
            create_nav_button(self, "Settings", Page::Settings),
        ];
        // All Pages of this page can be found here :3
        let content: Element<_> = match self.current_page {
            Page::Home => column![
                create_layout()
            ]
            .spacing(0)
            .height(Length::Fill)
            .into(),

            Page::Controls => column![
                text("Controls").size(30).style(theme::Text::Color(ACCENT)),
            ]
            .spacing(10)
            .height(Length::FillPortion(1))
            .into(),

            Page::Sensors => column![
                text("Sensors").size(30).style(theme::Text::Color(ACCENT)),
                text("Sensor data here").style(theme::Text::Color(Color::WHITE)),
            ]
            .spacing(10)
            .height(Length::FillPortion(1))
            .into(),

            Page::Think => column![
                text("Think").size(30).style(theme::Text::Color(ACCENT)),
                text("Thinking module").style(theme::Text::Color(Color::WHITE)),
            ]
            .spacing(10)
            .height(Length::FillPortion(1))
            .into(),

            Page::Settings => column![
                text("Settings").size(30).style(theme::Text::Color(ACCENT)),
                text("Settings panel").style(theme::Text::Color(Color::WHITE)),
            ]
            .spacing(10)
            .height(Length::FillPortion(1))
            .into(),

            Page::Socials => column![
                text("Settings").size(30).style(theme::Text::Color(ACCENT)),
                text("Settings panel").style(theme::Text::Color(Color::WHITE)),
            ]
            .spacing(10)
            .height(Length::FillPortion(1))
            .into(),

            Page::News => column![
                text("Settings").size(30).style(theme::Text::Color(ACCENT)),
                text("Settings panel").style(theme::Text::Color(Color::WHITE)),
            ]
            .spacing(10)
            .height(Length::FillPortion(1))
            .into(),

            Page::Games => column![
                text("Settings").size(30).style(theme::Text::Color(ACCENT)),
                text("Settings panel").style(theme::Text::Color(Color::WHITE)),
            ]
            .spacing(10)
            .height(Length::FillPortion(1))
            .into(),

            Page::Utils => column![
                text("Settings").size(30).style(theme::Text::Color(ACCENT)),
                text("Settings panel").style(theme::Text::Color(Color::WHITE)),
            ]
            .spacing(10)
            .height(Length::FillPortion(1))
            .into(),

            Page::Music => column![
                text("Settings").size(30).style(theme::Text::Color(ACCENT)),
                text("Settings panel").style(theme::Text::Color(Color::WHITE)),
            ]
            .spacing(10)
            .height(Length::FillPortion(1))
            .into(),

            Page::Messages => column![
                text("Settings").size(30).style(theme::Text::Color(ACCENT)),
                text("Settings panel").style(theme::Text::Color(Color::WHITE)),
            ]
            .spacing(10)
            .height(Length::FillPortion(1))
            .into(),

            Page::Infos => column![
                text("Settings").size(30).style(theme::Text::Color(ACCENT)),
                text("Settings panel").style(theme::Text::Color(Color::WHITE)),
            ]
            .spacing(10)
            .height(Length::FillPortion(1))
            .into(),
        };

        container(
            row![menu, container(content).padding(0)]
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Custom(Arc::new(iced::theme::Custom::new("CustomTheme".to_string(), Palette {
            background: BACKGROUND,
            text: Color::WHITE,
            primary: ACCENT,
            success: Color::from_rgb(0.0, 1.0, 0.0),
            danger: Color::from_rgb(1.0, 0.0, 0.0),
        })))
    }

    fn style(&self) -> iced::theme::Application {
        iced::theme::Application::Custom(Box::new(CustomApplicationStyle))
    }
}

fn create_nav_button<'a>(app: &'a App, label: &'a str, page: Page) -> Element<'a, Message> {
    button(text(label))
        .style(theme::Button::Custom(Box::new(CustomButtonStyle {
            is_active: app.current_page == page
        })))
        .width(Length::Fixed(170.0))
        .padding(20)
        .height(Length::FillPortion(1))
        .on_press(Message::Navigate(page))
        .into()
}

//Widgets of the Home page
fn create_layout() -> Element<'static, Message> {
    let handle = svg::Handle::from_memory(include_bytes!("../sources/msg-icon.svg").as_slice());
    let socials = svg::Handle::from_memory(include_bytes!("../sources/Socials.svg").as_slice());
    let games = svg::Handle::from_memory(include_bytes!("../sources/Games.svg").as_slice());
    let news = svg::Handle::from_memory(include_bytes!("../sources/News.svg").as_slice());
    let utils = svg::Handle::from_memory(include_bytes!("../sources/Utils.svg").as_slice());
    let music = svg::Handle::from_memory(include_bytes!("../sources/Music.svg").as_slice());
    let infos = svg::Handle::from_memory(include_bytes!("../sources/Infos.svg").as_slice());
    let row1 = row![
        button(
            container(
                svg(socials)
                    .width(Length::FillPortion(1))
                    .height(Length::Fill),
            )
            .padding(10),
        )
        .style(theme::Button::Custom(Box::new(CustomButtonStyle {is_active: true,})))
        .on_press(Message::Navigate(Page::Socials))
        .width(Length::FillPortion(1))
        .height(Length::Fill),

        button(
            container(
                svg(music)
                    .width(Length::FillPortion(2))
                    .height(Length::Fill),
            )
            .padding(10),
        )
        .style(theme::Button::Custom(Box::new(CustomButtonStyle {is_active: true,})))
        .on_press(Message::Navigate(Page::Music))
        .width(Length::FillPortion(1))
        .height(Length::Fill),
    ]
    .spacing(10)
    .width(Length::Fill)
    .height(Length::FillPortion(1));

    let row2 = row![
        button(
            container(
                svg(news)
                    .width(Length::FillPortion(1))
                    .height(Length::Fill),
            )
            .padding(10),
        )
        .style(theme::Button::Custom(Box::new(CustomButtonStyle {is_active: true,})))
        .on_press(Message::Navigate(Page::News))
        .width(Length::FillPortion(1))
        .height(Length::Fill),
        button(
            container(
                svg(games)
                    .width(Length::FillPortion(1))
                    .height(Length::Fill),
            )
            .padding(10),
        )
        .style(theme::Button::Custom(Box::new(CustomButtonStyle {is_active: true,})))
        .on_press(Message::Navigate(Page::Games))
        .width(Length::FillPortion(1))
        .height(Length::Fill),
        button(
            container(
                svg(utils)
                    .width(Length::FillPortion(1))
                    .height(Length::Fill),
            )
            .padding(10),
        )
        .style(theme::Button::Custom(Box::new(CustomButtonStyle {is_active: true,})))
        .on_press(Message::Navigate(Page::Utils))
        .width(Length::FillPortion(1))
        .height(Length::Fill),
    ]
    .spacing(10)
    .width(Length::Fill)
    .height(Length::FillPortion(1));

    let row3 = row![
        button(
            container(
                svg(handle)
                    .width(Length::FillPortion(2))
                    .height(Length::Fill),
            )
            .padding(10),
        )
        .style(theme::Button::Custom(Box::new(CustomButtonStyle {is_active: true,})))
        .on_press(Message::Navigate(Page::Messages))
        .width(Length::FillPortion(2))
        .height(Length::Fill),
        button(
            container(
                svg(infos)
                    .width(Length::FillPortion(1))
                    .height(Length::Fill),
            )
            .padding(10),
        )
        .style(theme::Button::Custom(Box::new(CustomButtonStyle {is_active: true,})))
        .on_press(Message::Navigate(Page::Infos))
        .width(Length::FillPortion(1))
        .height(Length::Fill),
    ]
    .spacing(10)
    .width(Length::Fill)
    .height(Length::FillPortion(1));

    container(
        column![row1, row2, row3]
            .spacing(10)
            .height(Length::Fill)
            .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

// Function to handle receiving username updates
pub fn sync(rx: Receiver<String>) {
    loop {
        match rx.recv() {
            Ok(value) => {
                println!("GUI received username: {}", value);
                let mut user_name = USER_NAME.lock().unwrap();
                *user_name = value;
            }
            Err(_) => break,
        }
    }
}
pub fn run() -> iced::Result {
    App::run(Settings::default())
}
