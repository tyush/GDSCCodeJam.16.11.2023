use iced::{Length, Pixels, Sandbox, Settings};

fn main() {
    println!("Hello, world!");
    AppState::run(Settings::default()).unwrap();
}

struct AppState {
    stick_counts: Vec<isize>,
    sticks: isize,
    curr_turn: usize,
    lost: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
enum AppMsg {
    Sticks(isize, usize),
    AddPlayer,
}

impl Sandbox for AppState {
    type Message = AppMsg;

    fn new() -> Self {
        AppState {
            curr_turn: 0,
            sticks: 20,
            lost: None,
            stick_counts: vec![0, 0],
        }
    }

    fn title(&self) -> String {
        format!("sticks")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            AppMsg::Sticks(amt, player) => {
                if self.sticks <= amt {
                    self.lost = player.into();
                    return;
                }
                self.stick_counts[player] += amt;
                self.sticks -= amt;
                self.curr_turn += 1;
                self.curr_turn %= self.stick_counts.len();
            }
            AppMsg::AddPlayer => self.stick_counts.push(0),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        use iced::widget as w;

        if let Some(loser) = self.lost {
            w::Text::new(format!("Player {loser} lost!!!"))
                .size(Pixels(98.))
                .into()
        } else {
            w::Column::new()
                .height(Length::Fill)
                .push(w::Row::new().push(w::Button::new("Add Player").on_press(AppMsg::AddPlayer)))
                .push({
                    let mut players = w::Row::new();
                    for (i, score) in self.stick_counts.iter().enumerate() {
                        players = players
                            .push(w::text(format!("Player {i} - Score {score}")))
                            .push(w::text("     "));
                    }
                    players
                })
                .push(w::Text::new(format!("Sticks in Pile - {}", self.sticks)).size(Pixels(28.)))
                .push(w::Text::new(format!("Player {} Turn", self.curr_turn)).size(Pixels(28.)))
                .push(
                    w::Row::new()
                        .push(
                            w::Button::new("Pickup 1 Stick")
                                .on_press(AppMsg::Sticks(1, self.curr_turn)),
                        )
                        .push(
                            w::Button::new("Pickup 2 Stick")
                                .on_press(AppMsg::Sticks(2, self.curr_turn)),
                        )
                        .push(
                            w::Button::new("Pickup 3 Stick")
                                .on_press(AppMsg::Sticks(3, self.curr_turn)),
                        ),
                )
                .into()
        }
    }
}
