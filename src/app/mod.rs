use iced::alignment::Horizontal;
use iced::executor::Default;
use iced::time;
use iced::widget::Button;
use iced::widget::Column;
use iced::widget::Container;
use iced::widget::Row;
use iced::widget::Rule;
use iced::widget::Text;
use iced::widget::TextInput;
use iced::Application;
use iced::Command;
use iced::Element;
use iced::Length;
use iced::Subscription;
use iced::Theme;
use serde::{Deserialize, Serialize};
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
    pub fn make_navigation<'a>(&self) -> Row<'a, WorkoutMessage> {
        let mut navigation = Row::new();
        navigation = navigation.push(
            Button::new("Previous")
                .on_press(WorkoutMessage::Previous)
                .width(200),
        );
        navigation = navigation.push(
            Button::new("Next")
                .on_press(WorkoutMessage::Next)
                .width(200),
        );
        navigation.width(Length::Fill).spacing(300).padding(100)
    }

    pub fn next(&mut self) {
        match self.status {
            AppStatus::Building => {
                if self.exercises.len() == 0
                    || self.exercises.iter().any(|ex| {
                        ex.sets == 0
                            || ex.reps == 0
                            || ex.progression.0 == 0
                            || ex.progression.1 == 0
                            || ex.rest.as_secs() == 0
                    })
                {
                    ()
                } else {
                    self.status = AppStatus::Exercising;
                    std::fs::write(
                        "exercises.json",
                        serde_json::to_string(&self.exercises).unwrap(),
                    );
                }
            }
            AppStatus::Exercising => {
                    self.status = AppStatus::Resting;
                    self.last_update = Instant::now();
                    self.rest_start = Instant::now();
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Exercise {
    name: String,
    progression: (usize, usize),
    reps: usize,
    sets: usize,
    rest: Duration,
}
#[derive(Debug, Clone)]
pub enum Field {
    Name,
    Progression,
    Reps,
    Sets,
    Rest,
}

impl std::default::Default for Exercise {
    fn default() -> Self {
        Exercise {
            name: "Exercise".to_string(),
            progression: (5, 12),
            reps: 5,
            sets: 3,
            rest: Duration::new(60, 0),
        }
    }
}
fn no_zero(n: usize) -> String {
    if n == 0 {
        String::new()
    } else {
        n.to_string()
    }
}
fn no_zero_f64(n: f64) -> String {
    if n == 0.0 {
        String::new()
    } else {
        n.to_string()
    }
}
impl Application for WorkoutApp {
    type Message = WorkoutMessage;

    fn new(flags: <WorkoutApp as iced::Application>::Flags) -> (Self, Command<WorkoutMessage>) {
        let saved_exercises = std::fs::read_to_string("exercises.json");
        let mut exercises = vec![Exercise::default()];
        if let Ok(json_file) = std::fs::read_to_string("exercises.json") {
            exercises = serde_json::from_str(&json_file).unwrap();
        }

        (
            WorkoutApp {
                status: AppStatus::Building,
                exercises,
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
            WorkoutMessage::UpdateText { index, field, text } => {
                if let Some(e) = self.exercises.get_mut(index) {
                    match field {
                        Field::Name => e.name = text,
                        Field::Progression => {
                            if text.contains("-") {
                                let mut split = text.split("-");
                                let starting = split.next().unwrap().parse().unwrap_or(0);
                                let ending = split.next().unwrap().parse().unwrap_or(0);
                                e.progression = (starting, ending);
                            }
                        }
                        Field::Reps => {
                            e.reps = text.parse().unwrap_or(0);
                        }
                        Field::Sets => {
                            e.sets = text.parse().unwrap_or(0);
                        }
                        Field::Rest => {
                            e.rest = Duration::from_secs_f64(text.parse().unwrap_or(0.0));
                        }
                    }
                }
            }
            WorkoutMessage::AddExercise => {
                self.exercises.push(Exercise::default());
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
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let style = style::AppStyle::default();

        match self.status {
            AppStatus::Building => {
                let mut column = Column::new();
                let content = Text::new("Current Exercises")
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center);

                column = column.push(content);
                let mut header = Row::new();
                header = header.push(Text::new("Name").width(300));
                header = header.push(Text::new("Progression").width(style.table_column_width));
                header = header.push(Text::new("Reps").width(100));
                header = header.push(Text::new("Sets").width(100));
                header = header.push(Text::new("Rest").width(100));
                header =
                    header.push(Button::new("New Exercise").on_press(WorkoutMessage::AddExercise));
                column = column.push(header);
                column = column.push(Rule::horizontal(10));

                for (i, exercise) in self.exercises.iter().enumerate() {
                    let mut row = Row::new().padding(style.table_row_padding);
                    row = row.push(
                        TextInput::new(&exercise.name, &exercise.name)
                            .width(300)
                            .on_input(move |text| WorkoutMessage::UpdateText {
                                index: i,
                                field: Field::Name,
                                text,
                            }),
                    );
                    let text_value = format!(
                        "{}-{}",
                        no_zero(exercise.progression.0),
                        no_zero(exercise.progression.1)
                    );

                    row = row.push(
                        TextInput::new(&text_value, &text_value)
                            .width(style.table_column_width)
                            .on_input(move |text| WorkoutMessage::UpdateText {
                                index: i,
                                field: Field::Progression,
                                text,
                            }),
                    );
                    row = row.push(
                        TextInput::new(&no_zero(exercise.reps), &no_zero(exercise.reps))
                            .width(100)
                            .on_input(move |text| WorkoutMessage::UpdateText {
                                index: i,
                                field: Field::Reps,
                                text,
                            }),
                    );
                    row = row.push(
                        TextInput::new(&no_zero(exercise.sets), &no_zero(exercise.sets))
                            .width(100)
                            .on_input(move |text| WorkoutMessage::UpdateText {
                                index: i,
                                field: Field::Sets,
                                text,
                            }),
                    );
                    row = row.push(
                        TextInput::new(
                            &no_zero_f64(exercise.rest.as_secs_f64()),
                            &no_zero_f64(exercise.rest.as_secs_f64()),
                        )
                        .width(100)
                        .on_input(move |text| WorkoutMessage::UpdateText {
                            index: i,
                            field: Field::Rest,
                            text,
                        }),
                    );
                    row =
                        row.push(Button::new("Delete").on_press(WorkoutMessage::RemoveExercise(i)));

                    column = column.push(row);
                }
                column = column.push(self.make_navigation());
                Container::new(column).center_x().center_y().into()
            }
            AppStatus::Exercising => {
                let mut column = Column::new();
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
                    .height(100)
                    .horizontal_alignment(Horizontal::Center),
                );

                column = column.push(self.make_navigation());
                Container::new(column).center_x().center_y().into()
            }
            AppStatus::Resting => {
                let mut column = Column::new();
                let current_rest = self.exercises[self.current_exercise].rest.as_secs_f64();
                let current_seconds = (self.last_update - self.rest_start).as_secs_f64();
                let seconds_left = current_rest - current_seconds;
                let content = Text::new(if seconds_left <= 0.0 {
                    "Done".to_string()
                } else {
                    format!("{}", seconds_left.round() as usize)
                })
                .width(Length::Fill)
                .height(130)
                .horizontal_alignment(Horizontal::Center);
                column = column.push(
                    Text::new("Rest")
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Center),
                );
                column = column.push(content);

                column = column.push(self.make_navigation());
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

    fn theme(&self) -> Self::Theme {
        Self::Theme::default()
    }

    fn style(&self) -> <Self::Theme as iced::application::StyleSheet>::Style {
        <Self::Theme as iced::application::StyleSheet>::Style::default()
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }
}

#[derive(Debug, Clone)]
pub enum WorkoutMessage {
    Next,
    Previous,
    UpdateText {
        index: usize,
        field: Field,
        text: String,
    },
    AddExercise,
    RemoveExercise(usize),
    UpdateTimer(Instant),
}
