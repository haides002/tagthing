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

    let _ = iced::application("Tagthing", ui::Tagthing::update, ui::Tagthing::view).run_with(
        || -> (Tagthing, Task<_>) {
            (
                Tagthing::new(PathBuf::from(
                    "/home/inus/downloads/Anime-Girls-Holding-Programming-Books",
                )),
                Task::none(),
            )
        },
    );

    dbg!(parse_date("2022-03-02T22:37:46"));
}
