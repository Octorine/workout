use iced::executor::Default;
use iced::time;
use iced::widget::Button;
use iced::widget::Column;
use iced::widget::Container;
use iced::widget::Row;
use iced::widget::Text;
use iced::Application;
use iced::Command;
use iced::Element;
use iced::Subscription;
use iced::Theme;
use std::time::{Duration, Instant};
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
                let exercise = self.exercises[self.current_exercise].clone();
                self.current_set = self.current_set.saturating_sub(1);
                if self.current_set <= 0 {
                    self.current_set = exercise.sets - 1;
                    self.current_exercise = self.current_exercise.saturating_sub(1);
                }
                if self.current_exercise <= 0 {
                    self.current_exercise = 0;
                    self.current_set = 0;
                    self.status = AppStatus::Building;
                } else {
                    self.status = AppStatus::Resting;
                    self.last_update = Instant::now();
                    self.rest_start = Instant::now();
                }
            }
            AppStatus::Resting => {}
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
    amount: String,
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
                        amount: "10 s".to_string(),
                        sets: 3,
                        rest: Duration::new(60, 0),
                    },
                    Exercise {
                        name: "Static Deadbug Hold".to_string(),
                        amount: "15 s".to_string(),
                        sets: 3,
                        rest: Duration::new(60, 0),
                    },
                    Exercise {
                        name: "Squats".to_string(),
                        amount: "8 reps".to_string(),
                        sets: 2,
                        rest: Duration::new(60, 0),
                    },
                    Exercise {
                        name: "Glute Bridges".to_string(),
                        amount: "10 reps".to_string(),
                        sets: 2,
                        rest: Duration::new(60, 0),
                    },
                    Exercise {
                        name: "Rows".to_string(),
                        amount: "12 reps".to_string(),
                        sets: 2,
                        rest: Duration::new(60, 0),
                    },
                    Exercise {
                        name: "Pushups".to_string(),
                        amount: "12 reps".to_string(),
                        sets: 2,
                        rest: Duration::new(60, 0),
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
                    amount: "10 reps".to_string(),
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
        match self.status {
            AppStatus::Building => {
                let mut row = Row::new();
                let mut column = Column::new();
                let prev_button = Button::new("Previous").on_press(WorkoutMessage::Previous);
                let next_button = Button::new("Next").on_press(WorkoutMessage::Next);
                let content = Text::new("Current Exercises");
                column = column.push(content);
                for exercise in self.exercises.iter() {
                    column = column.push(Text::new(format!(
                        "{}\nAmount: {}, Sets: {}, Rest: {}",
                        exercise.name,
                        exercise.amount,
                        exercise.sets,
                        exercise.rest.as_secs_f64()
                    )));
                }
                row = row.push(prev_button);
                row = row.push(next_button);
                column = column.push(row);
                Container::new(column).into()
            }
            AppStatus::Exercising => {
                let mut column = Column::new();
                let prev_button = Button::new("Previous").on_press(WorkoutMessage::Previous);
                let next_button = Button::new("Next").on_press(WorkoutMessage::Next);
                let mut row = Row::new();
                let content = Text::new(format!(
                    "Current Exercise: {}, Amount: {}, Set {} of {}",
                    self.exercises[self.current_exercise].name,
                    self.exercises[self.current_exercise].amount,
                    self.current_set + 1, // 1-indexed for humans
                    self.exercises[self.current_exercise].sets
                ));
                column = column.push(content);

                row = row.push(prev_button);
                row = row.push(next_button);
                column = column.push(row);
                Container::new(column).into()
            }
            AppStatus::Resting => {
                let mut column = Column::new();
                let prev_button = Button::new("Previous").on_press(WorkoutMessage::Previous);
                let next_button = Button::new("Next").on_press(WorkoutMessage::Next);
                let mut row = Row::new();
                let current_rest = self.exercises[self.current_exercise].rest.as_secs_f64();
                let current_seconds = (self.last_update - self.rest_start).as_secs_f64();
                let seconds_left = current_rest - current_seconds;
                let content = Text::new(if seconds_left <= 0.0 {
                    "Done".to_string()
                } else {
                    format!("{}", seconds_left.round() as usize)
                });
                column = column.push(content);

                row = row.push(prev_button);
                row = row.push(next_button);
                column = column.push(row);
                Container::new(column).into()
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
