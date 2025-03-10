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
    details_view: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectImage(usize),
    UpdateQuery(String),
    TagSelected(String),
    FilesRead(Vec<crate::file::File>),
}

impl Tagthing {
    pub fn new(path: PathBuf) -> Self {
        Tagthing {
            ..Default::default()
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SelectImage(index) => self.selected = vec![index],
            Message::UpdateQuery(query) => self.query = query,
            Message::TagSelected(tag) => self.query = tag,
            Message::FilesRead(files) => {
                self.tag_cache = TagCache::new(&files);
                self.files = files;
            },
        }
    }

    pub fn view(&self) -> Element<Message> {
        println!("view was called");
        const IMAGES_PER_ROW: usize = 4;
        let filter_view = container(column![TextInput::new("Search ...", &self.query)
            .on_input(|input: String| Message::UpdateQuery(input))]);

        let gallery_view = container(scrollable({
            let mut images = Column::new();
            let mut image_row: Row<_> = Row::new();

            for (i, file) in self.files.iter().enumerate() {
                image_row = image_row.push(
                    button(
                        image(&file.path)
                            .content_fit(iced::ContentFit::Contain)
                    )
                    .on_press(Message::SelectImage(i))
                    .style(|_, _| button::Style::default())
                    .width(Length::Fill)
                    .padding(0),
                );
                if (i + 1) % IMAGES_PER_ROW == 0 {
                    images = images.push(image_row);
                    image_row = Row::with_capacity(4);
                }
            }
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

        row![
            filter_view.width(FillPortion(1)),
            gallery_view.width(FillPortion(3)),
            details_view.width(FillPortion(2))
        ]
        .into()
    }
}
