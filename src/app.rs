use iced::widget::Container;
use iced::widget::Text;
use iced::Application;
use iced::Element;
use iced::Sandbox;

#[derive(Debug)]
pub struct WorkoutApp {
    status: AppStatus,
    exercises: Vec<Exercise>,
    current_exercise: usize,
    current_set: usize,
    rest_start: RestTime,
}
#[derive(Debug)]
pub enum AppStatus {
    Building,
    Exercising,
}

#[derive(Debug)]
pub struct Exercise {
    name: String,
    reps: usize,
    sets: usize,
    rest: RestTime,
}

type RestTime = f32;

impl Sandbox for WorkoutApp {
    type Message = WorkoutMessage;

    fn new() -> Self {
        // TODO
        WorkoutApp {
            status: AppStatus::Building,
            exercises: vec![],
            current_exercise: 0,
            current_set: 0,
            rest_start: 0.0,
        }
    }

    fn title(&self) -> String {
        "Workout Helper App".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        //TODO
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match self.status {
            AppStatus::Building => Container::new(Text::new("Welcome to the app")).into(),
            AppStatus::Exercising => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum WorkoutMessage {
    StartExercising,
    Pause,
    UnPause,
    NextExcercise,
    NextSet,
    UpdateText,
    AddExercise,
    RemoveExercise,
}
