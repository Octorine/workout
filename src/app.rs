use iced::executor::Default;
use iced::widget::Container;
use iced::widget::Text;
use iced::Application;
use iced::Command;
use iced::Element;
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
#[derive(Debug)]
pub enum AppStatus {
    Building,
    Exercising,
    Resting,
}

#[derive(Debug)]
pub struct Exercise {
    name: String,
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
                exercises: vec![],
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
            WorkoutMessage::StartExercising => {
                self.status = AppStatus::Exercising;
            }
            WorkoutMessage::StartResting => {
                self.status = AppStatus::Resting;
                self.rest_start = Instant::now();
            }
            WorkoutMessage::Next => {
                if let Some(e) = self.exercises.get(self.current_exercise) {
                    self.current_set += 1;
                    if self.current_set >= e.sets {
                        self.current_set = 0;
                        self.current_exercise = (self.current_exercise + 1) % self.exercises.len();
                    }
                }
            }
            WorkoutMessage::UpdateText { index, text } => {
                if let Some(e) = self.exercises.get_mut(index) {
                    e.name = text;
                }
            }
            WorkoutMessage::AddExercise => {
                self.exercises.push(Exercise {
                    name: "".to_string(),
                    reps: 0,
                    sets: 0,
                    rest: Duration::from_secs(60),
                });
            }
            WorkoutMessage::RemoveExercise(index) => {
                self.exercises.remove(index);
            }
            WorkoutMessage::Previous => {
                if self.current_set == 0 {
                    if self.current_exercise > 0 {
                        self.current_exercise -= 1;
                        self.current_set = self.exercises[self.current_exercise].sets - 1;
                    }
                }
            }
            WorkoutMessage::UpdateTimer(tick) => {
                self.last_update = tick;
            }
        }
        Command::none()
        //TODO
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match self.status {
            AppStatus::Building => Container::new(Text::new("Welcome to the app")).into(),
            AppStatus::Exercising => todo!(),
            AppStatus::Resting => todo!(),
        }
    }

    type Executor = Default;

    type Theme = Theme;

    type Flags = ();
}

#[derive(Debug)]
pub enum WorkoutMessage {
    StartExercising,
    StartResting,
    Next,
    Previous,
    UpdateText { index: usize, text: String },
    AddExercise,
    RemoveExercise(usize),
    UpdateTimer(Instant),
}
