use std::f32::consts::E;

mod app;
mod window;

fn main() {
    if let Err(e) = app::run() {
        eprintln!("Ошибка: {}", E);
    }
}