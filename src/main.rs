use iced::{Application, Settings};
mod app;

fn main() {
    app::WorkoutApp::run(Settings {
        default_text_size: 30.0,
        ..Default::default()
    })
    .unwrap();
}
