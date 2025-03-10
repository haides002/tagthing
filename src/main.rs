use std::path::PathBuf;

use iced::Task;
use ui::Tagthing;
use utils::parse_date;

mod file;
#[macro_use]
mod utils;
mod tagcache;
mod ui;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let gallery_path = std::env::args().collect::<Vec<String>>().iter().nth(1).expect("No gallery path specified").clone();

    let _ = iced::application("Tagthing", ui::Tagthing::update, ui::Tagthing::view).run_with(
        || -> (Tagthing, Task<_>) {
            (
                Tagthing::default(),
                Task::perform(crate::file::File::read_dir(PathBuf::from(gallery_path)), crate::ui::Message::FilesRead),
            )
        },
    );
}
