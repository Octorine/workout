use iced::alignment::Horizontal;
use iced::executor::Default;
use iced::time;
use iced::widget::Button;
use iced::widget::Column;
use iced::widget::Container;
use iced::widget::Row;
use iced::widget::Rule;
use iced::widget::Text;
use iced::Application;
use iced::Command;
use iced::Element;
use iced::Length;

use iced::Subscription;
use iced::Theme;
use std::time::{Duration, Instant};

mod style;

#[derive(Debug)]
pub struct WorkoutApp {
    status: AppStatus,
    exercises: Vec<Exercise>,
    current_exercise: usize,
    current_set: usize,
    rest_start: Instant,
    last_update: Instant,
}

impl WorkoutApp {
    pub fn next(&mut self) {
        match self.status {
            AppStatus::Building => self.status = AppStatus::Exercising,
            AppStatus::Exercising => {
                if self.current_set >= self.exercises[self.current_exercise].sets - 1 {
                    self.current_set = 0;
                    self.current_exercise += 1;
                    if self.current_exercise > self.exercises.len() - 1 {
                        self.status = AppStatus::Building;
                        self.current_exercise = 0;
                        self.current_set = 0;
                    }
                } else {
                    self.status = AppStatus::Resting;
                    self.last_update = Instant::now();
                    self.rest_start = Instant::now();
                }
            }
            AppStatus::Resting => {
                let exercise = self.exercises[self.current_exercise].clone();
                self.current_set += 1;
                if self.current_set >= exercise.sets {
                    self.current_set = 0;
                    self.current_exercise += 1;
                }
                if self.current_exercise >= self.exercises.len() {
                    self.current_exercise = 0;
                    self.current_set = 0;

                    self.status = AppStatus::Building;
                } else {
                    self.status = AppStatus::Exercising;
                }
            }
        }
    }
    pub fn prev(&mut self) {
        match self.status {
            AppStatus::Building => (),
            AppStatus::Exercising => {
                if self.current_exercise == 0 && self.current_set == 0 {
                    self.status = AppStatus::Building;
                } else if self.current_set == 0 {
                    self.current_exercise -= 1;
                    self.current_set = self.exercises[self.current_exercise].sets - 1;
                } else {
                    self.current_set -= 1;
                }
            }
            AppStatus::Resting => {
                self.status = AppStatus::Exercising;
            }
        }
    }
}

#[derive(Debug)]
pub enum AppStatus {
    Building,
    Exercising,
    Resting,
}

#[derive(Debug, Clone)]
pub struct Exercise {
    name: String,
    progression: (usize, usize),
    reps: usize,
    sets: usize,
    rest: Duration,
}

impl Application for WorkoutApp {
    type Message = WorkoutMessage;

    fn new(flags: <WorkoutApp as iced::Application>::Flags) -> (Self, Command<WorkoutMessage>) {
        // TODO
        (
            WorkoutApp {
                status: AppStatus::Building,
                exercises: vec![
                    Exercise {
                        name: "Static Birddog Hold".to_string(),
                        progression: (6, 10),
                        reps: 6,
                        sets: 3,
                        rest: Duration::new(60, 0),
                    },
                    Exercise {
                        name: "Static Deadbug Hold".to_string(),
                        progression: (6, 10),
                        reps: 6,
                        sets: 3,
                        rest: Duration::new(60, 0),
                    },
                    Exercise {
                        name: "Squats".to_string(),
                        progression: (8, 15),
                        reps: 8,
                        sets: 3,
                        rest: Duration::new(90, 0),
                    },
                    Exercise {
                        name: "Glute Bridges".to_string(),
                        progression: (8, 15),
                        reps: 8,
                        sets: 3,
                        rest: Duration::new(90, 0),
                    },
                    Exercise {
                        name: "Chest Rows".to_string(),
                        progression: (5, 12),
                        reps: 5,
                        sets: 3,
                        rest: Duration::new(120, 0),
                    },
                    Exercise {
                        name: "Chest Pushups".to_string(),
                        progression: (5, 12),
                        reps: 5,
                        sets: 3,
                        rest: Duration::new(120, 0),
                    },
                ],
                current_exercise: 0,
                current_set: 0,
                rest_start: Instant::now(),
                last_update: Instant::now(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Workout Helper App".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<WorkoutMessage> {
        match message {
            WorkoutMessage::Next => self.next(),
            WorkoutMessage::UpdateText { index, text } => {
                if let Some(e) = self.exercises.get_mut(index) {
                    e.name = text;
                }
            }
            WorkoutMessage::AddExercise => {
                self.exercises.push(Exercise {
                    name: "".to_string(),
                    progression: (5, 12),
                    reps: 5,
                    sets: 0,
                    rest: Duration::from_secs(60),
                });
            }
            WorkoutMessage::RemoveExercise(index) => {
                self.exercises.remove(index);
            }
            WorkoutMessage::Previous => self.prev(),
            WorkoutMessage::UpdateTimer(tick) => {
                self.last_update = tick;
            }
        }
        Command::none()
        //TODO
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let style = style::AppStyle::default();

        match self.status {
            AppStatus::Building => {
                let mut navigation = Row::new();
                let mut column = Column::new();
                let prev_button = Button::new("Previous").on_press(WorkoutMessage::Previous);
                let next_button = Button::new("Next").on_press(WorkoutMessage::Next);
                let content = Text::new("Current Exercises")
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center);
                column = column.push(content);
                let mut header = Row::new();
                header = header.push(Text::new("Name").width(style.table_column_width));
                header = header.push(Text::new("Progression").width(style.table_column_width));
                header = header.push(Text::new("Reps").width(style.table_column_width));
                header = header.push(Text::new("Sets").width(style.table_column_width));
                header = header.push(Text::new("Rest").width(style.table_column_width));
                column = column.push(header);
                column = column.push(Rule::horizontal(10));

                for exercise in self.exercises.iter() {
                    let mut row = Row::new().padding(style.table_row_padding);
                    row = row.push(
                        Text::new(exercise.name.clone())
                            .width(200)
                            .width(style.table_column_width),
                    );
                    row = row.push(
                        Text::new(format!(
                            "{}-{}",
                            exercise.progression.0, exercise.progression.1
                        ))
                        .width(style.table_column_width),
                    );
                    row = row
                        .push(Text::new(exercise.reps.to_string()).width(style.table_column_width));
                    row = row
                        .push(Text::new(exercise.sets.to_string()).width(style.table_column_width));
                    row = row.push(
                        Text::new(exercise.rest.as_secs_f64().to_string())
                            .width(style.table_column_width),
                    );

                    column = column.push(row);
                }
                navigation = navigation.push(prev_button);
                navigation = navigation.push(next_button);
                column = column.push(navigation);
                Container::new(column).center_x().center_y().into()
            }
            AppStatus::Exercising => {
                let mut column = Column::new();
                let prev_button = Button::new("Previous").on_press(WorkoutMessage::Previous);
                let next_button = Button::new("Next").on_press(WorkoutMessage::Next);
                let mut navigation = Row::new();
                column = column.push(
                    Text::new(format!(
                        "Current Exercise: {}",
                        self.exercises[self.current_exercise].name
                    ))
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center),
                );
                column = column.push(
                    Text::new(format!(
                        "Amount: {}",
                        self.exercises[self.current_exercise].reps.to_string()
                    ))
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center),
                );
                column = column.push(
                    Text::new(format!(
                        "Set {} of {}",
                        self.current_set + 1, // 1-indexed for humans
                        self.exercises[self.current_exercise].sets
                    ))
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center),
                );

                navigation = navigation.push(prev_button);
                navigation = navigation.push(next_button);
                column = column.push(navigation);
                Container::new(column).center_x().center_y().into()
            }
            AppStatus::Resting => {
                let mut column = Column::new();
                let prev_button = Button::new("Previous").on_press(WorkoutMessage::Previous);
                let next_button = Button::new("Next").on_press(WorkoutMessage::Next);
                let mut navigation = Row::new();
                let current_rest = self.exercises[self.current_exercise].rest.as_secs_f64();
                let current_seconds = (self.last_update - self.rest_start).as_secs_f64();
                let seconds_left = current_rest - current_seconds;
                let content = Text::new(if seconds_left <= 0.0 {
                    "Done".to_string()
                } else {
                    format!("{}", seconds_left.round() as usize)
                })
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center);
                column = column.push(
                    Text::new("Rest")
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Center),
                );
                column = column.push(content);

                navigation = navigation.push(prev_button);
                navigation = navigation.push(next_button);
                column = column.push(navigation);
                Container::new(column).center_x().center_y().into()
            }
        }
    }

    type Executor = Default;

    type Theme = Theme;

    type Flags = ();

    fn subscription(&self) -> Subscription<Self::Message> {
        match self.status {
            AppStatus::Resting => {
                time::every(Duration::from_millis(99)).map(WorkoutMessage::UpdateTimer)
            }
            _ => Subscription::none(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum WorkoutMessage {
    Next,
    Previous,
    UpdateText { index: usize, text: String },
    AddExercise,
    RemoveExercise(usize),
    UpdateTimer(Instant),
}
