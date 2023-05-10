mod request;
use iced::widget::{button, column, text, TextInput};
use iced::{Alignment, Element, Sandbox, Settings};

struct BrowserWindow {
    contents: String
}

#[derive(Debug, Clone)]
enum Message {
    TextInputChanged(String),
}

impl Sandbox for BrowserWindow {
    type Message = Message;

    fn new() -> Self {
        BrowserWindow {
            contents: String::new()
        }
    }

    fn title(&self) -> String {
        String::from("ravenna")
    }

    fn update(&mut self, message: Message) {
        let Message::TextInputChanged(_url) = message;
        let data = request::get("veera.app", "/index.html", 443).unwrap();
        self.contents = data.body;
    }

    fn view(&self) -> Element<Message> {
        column![
            TextInput::new("This is the placeholder...", "Enter your URL")
                .on_input(Message::TextInputChanged)
                .padding(10),
            text(self.contents.clone()).size(50)
        ]
            .padding(20)
            .align_items(Alignment::Center)
            .into()

    }
}

// fn main() -> std::io::Result<()> {
//     let data = request::get("veera.app", "/index.html", 443).unwrap();
//     println!("status: {:?}", data.status);
//     println!("Header: {:#?}", data.header);
//     print!("{data}");
//     Ok(())
// }

fn main() -> iced::Result {
    BrowserWindow::run(Settings::default())
}
