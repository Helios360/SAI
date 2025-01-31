use iced::widget::{button, column, container, row, text};
use iced::{Element, Length, Sandbox, Settings};
use iced::theme::{self, Theme};
use iced::{application, Color};
use iced::theme::Application;

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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Page {
    Home,
    Settings,
    About,
}

pub struct App {
    current_page: Page,
    counter: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Navigate(Page),
    Increment,
    Decrement,
}

struct CustomApplicationStyle;

impl application::StyleSheet for CustomApplicationStyle {
    type Style = Theme;

    fn appearance(&self, _: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: BACKGROUND,
            text_color: Color::WHITE,
        }
    }
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            current_page: Page::Home,
            counter: 0,
        }
    }

    fn title(&self) -> String {
        String::from("SAI -- Dashboard")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Navigate(page) => self.current_page = page,
            Message::Increment => self.counter += 1,
            Message::Decrement => self.counter -= 1,
        }
    }

    fn view(&self) -> Element<Message> {
        // Helper function to determine button style based on current page
        fn get_button_style(current: Page, button: Page) -> theme::Button {
            if current == button {
                theme::Button::Primary
            } else {
                theme::Button::Secondary
            }
        }

        let menu = column![
            button("Home")
                .style(get_button_style(self.current_page, Page::Home))
                .width(Length::Fixed(170.0))
                .padding(20)
                .on_press(Message::Navigate(Page::Home)),
            button("Settings")
                .style(get_button_style(self.current_page, Page::Settings))
                .width(Length::Fixed(170.0))
                .padding(20)
                .on_press(Message::Navigate(Page::Settings)),
            button("About")
                .style(get_button_style(self.current_page, Page::About))
                .width(Length::Fixed(170.0))
                .padding(20)
                .on_press(Message::Navigate(Page::About)),
        ];

        let content: Element<_> = match self.current_page {
            Page::Home => column![
                text("Home Page").size(30).style(theme::Text::Color(ACCENT)),
                text(format!("Counter: {}", self.counter)).style(theme::Text::Color(Color::WHITE)),
                button("Increment")
                    .style(theme::Button::Primary)
                    .on_press(Message::Increment),
                button("Decrement")
                    .style(theme::Button::Destructive)
                    .on_press(Message::Decrement),
            ]
            .spacing(10)
            .into(),

            Page::Settings => column![
                text("Settings Page")
                    .size(30)
                    .style(theme::Text::Color(ACCENT)),
                text("Settings content goes here...")
                    .style(theme::Text::Color(Color::WHITE)),
            ]
            .spacing(10)
            .into(),

            Page::About => column![
                text("About Page")
                    .size(30)
                    .style(theme::Text::Color(ACCENT)),
                text("About content goes here...")
                    .style(theme::Text::Color(Color::WHITE)),
            ]
            .spacing(10)
            .into(),
        };

        container(
            row![menu, container(content).padding(20)]
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::custom(theme::Palette {
            background: BACKGROUND,
            text: Color::WHITE,
            primary: ACCENT,
            success: Color::from_rgb(0.0, 0.8, 0.4),
            danger: Color::from_rgb(0.8, 0.0, 0.0),
        })
    }

    fn style(&self) -> iced::theme::Application {
        iced::theme::Application::Custom(Box::new(CustomApplicationStyle))
    }   
}

pub fn run() -> iced::Result {
    App::run(Settings::default())
}