use std::path::PathBuf;

use iced::Task;
use ui::Tagthing;
use utils::parse_date;

mod file;
#[macro_use]
mod utils;
mod tagcache;
mod ui;

fn main() {
    println!("Hello, world!");

    let gallery_path = std::env::args().collect::<Vec<String>>().iter().nth(1).expect("No gallery path specified").clone();

    let _ = iced::application("Tagthing", ui::Tagthing::update, ui::Tagthing::view).run_with(
        || -> (Tagthing, Task<_>) {
            (
                Tagthing::new(PathBuf::from(
                    gallery_path,
                )),
                Task::none(),
            )
        },
    );
}
