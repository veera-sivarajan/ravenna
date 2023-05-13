use iced::theme::{self, Theme};
use iced::widget::{column, text, TextInput};
use iced::{Alignment, Application, Element};
use iced::{Color, Command, Length, Settings, Subscription};

use crate::request;
use crate::url::parse_url;

#[derive(Default)]
struct Window {
    contents: String,
    user_input: String,
}

#[derive(Debug, Clone)]
enum Message {
    UserInputChanged(String),
    UserInputSubmitted,
}

impl Application for Window {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("ravenna")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::UserInputChanged(input) => {
                self.user_input = input;
                Command::none()
            }
            Message::UserInputSubmitted => {
                if let Some((website, path)) = parse_url(&self.user_input) {
                    println!("Requesting {website} at {path}");
                    self.contents = request::get(website, path, 443).to_string();
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            TextInput::new("Enter an URL", &self.user_input)
                .on_input(Message::UserInputChanged)
                .on_submit(Message::UserInputSubmitted)
                .padding(10),
            text(self.contents.to_string()).size(10)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}

pub fn start_browser() -> iced::Result {
    Window::run(Settings::default())
}
