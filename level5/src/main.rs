use iced::{Length, Sandbox};

fn main() {
    println!("Hello, world!");
}

struct AppState {
    sticks: Vec<isize>,
    curr_turn: usize,
}

#[derive(Debug, Clone, Copy)]
enum AppMsg {
    Sticks(isize),
    AddPlayer,
}

impl Sandbox for AppState {
    type Message = AppMsg;

    fn new() -> Self {
        AppState {
            curr_turn: 0,
            sticks: vec![0, 0],
        }
    }

    fn title(&self) -> String {
        format!("sticks")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            AppMsg::Sticks(_) => todo!(),
            AppMsg::AddPlayer => todo!(),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        use iced::widget as w;
        w::Column::new()
            .height(Length::Fill)
            .push(w::Row::new().push(w::Button::new("Add Player").on_press(AppMsg::AddPlayer)))
            .push({
                let mut players = w::Row::new();
                for (i, score) in self.sticks.iter() {
                    players = players.push(w::text(format!("Player {i} - Score {score}")))
                }
            })
            .into()
    }
}
