use std::path::PathBuf;

use iced::{
    widget::{
        button, column, container, image, row, scrollable, text, Button, Column, Image, Row,
        TextInput, lazy
    },
    Element,
    Length::{self, FillPortion},
};

use crate::{file::File, tagcache::TagCache};

#[derive(Debug, Default)]
pub struct Tagthing {
    tag_cache: TagCache,
    files: Vec<File>,
    selected: Vec<usize>,
    query: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectImage(usize),
    UpdateQuery(String),
    TagSelected(String),
}

impl Tagthing {
    pub fn new(path: PathBuf) -> Self {
        let files = crate::file::File::read_dir(path);

        Tagthing {
            tag_cache: crate::tagcache::TagCache::new(&files),
            files,
            ..Default::default()
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SelectImage(index) => self.selected = vec![index],
            Message::UpdateQuery(query) => self.query = query,
            Message::TagSelected(tag) => self.query = tag,
        }
    }

    pub fn view(&self) -> Element<Message> {
        const IMAGES_PER_ROW: usize = 4;
        let filter_view = container(column![TextInput::new("Search ...", &self.query)
            .on_input(|input: String| Message::UpdateQuery(input))]);

        let gallery_view = container(scrollable({
            let mut images = Column::new();
            let mut image_row: Row<_> = Row::new();
            println!("Starting to load images...");
            let now = std::time::Instant::now();

            for (i, file) in self.files.iter().enumerate() {
                image_row = image_row.push(
                    button(
                        //lazy(file, |f| { // lazy breaks the layout
                        //    image(&f.path)
                        //        .width(Length::Fill)
                        //        .content_fit(iced::ContentFit::Contain)
                        //    },
                        //)
                        image(&file.path)
                            .width(Length::Fill)
                            .content_fit(iced::ContentFit::Contain)
                    )
                    .on_press(Message::SelectImage(i))
                    .style(|_, _| button::Style::default())
                    .padding(0),
                );
                if (i + 1) % IMAGES_PER_ROW == 0 {
                    images = images.push(image_row);
                    image_row = Row::with_capacity(4);
                }
            }
            println!("Loading images took {} microseconds", now.elapsed().as_micros());
            images.push(image_row)
        }));

        let details_view = container(scrollable({
            let current_file: Option<&File> = {
                match self.selected.first() {
                    Some(i) => Some(&self.files[*i]),
                    None => None,
                }
            };

            match current_file {
                Some(file) => {
                    column![
                        text(file.path.to_str().unwrap_or_default()),
                        image(&file.path),
                        {
                            let mut tags = Row::new();
                            for tag in &file.tags {
                                tags = tags.push(
                                    button(&**tag).on_press(Message::TagSelected(tag.to_string())),
                                );
                            }
                            tags.wrap()
                        }
                    ]
                }
                None => column![text("Amogus!")],
            }
        }));

        println!("Returning view");
        row![
            filter_view.width(FillPortion(1)),
            gallery_view.width(FillPortion(3)),
            details_view.width(FillPortion(2))
        ]
        .into()
    }
}
