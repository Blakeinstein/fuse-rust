use iced::widget::{column, container, row, scrollable, text, text_input};
use iced::{Alignment, Color, Element, Length, Sandbox, Settings};
use once_cell::sync::Lazy;

use fuse_rust::{Fuse, SearchResult};

use std::ops::Range;

const BOOKS: &'static [&'static str] = &[
    "Angels & Demons",
    "Old Man's War",
    "The Lock Artist",
    "HTML5",
    "Right Ho Jeeves",
    "The Code of the Wooster",
    "Thank You Jeeves",
    "The DaVinci Code",
    "The Silmarillion",
    "Syrup",
    "The Lost Symbol",
    "The Book of Lies",
    "Lamb",
    "Fool",
    "Incompetence",
    "Fat",
    "Colony",
    "Backwards, Red Dwarf",
    "The Grand Design",
    "The Book of Samson",
    "The Preservationist",
    "Fallen",
    "Monster 1959"
];

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

pub fn main() -> iced::Result{
    Example::run(Settings::default())
}

struct Example<'a> {
    search_query: String,
    book_list: Vec<&'a str>,
    visible_books: Option<Vec<SearchResult>>,
    fuse_instance: Fuse,
}

#[derive(Debug, Clone)]
enum Message {
    SearchQuery(String),
}

impl Sandbox for Example<'_> {
    type Message = Message;

    fn new() -> Self {
        Self {
            search_query: String::default(),
            book_list: BOOKS.to_vec(),
            visible_books: None,
            fuse_instance: Fuse::default(),
        }
    }

    fn title(&self) -> String {
        String::from("Fuse-Rust search bar demo")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SearchQuery(query) => {
                self.search_query = query;
                self.visible_books = if !self.search_query.is_empty() { Some(self.fuse_instance.search_text_in_iterable(
                    &self.search_query,
                    self.book_list.iter()
                )) } else { None };
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let query_box = text_input(
            "Search query:",
            &self.search_query,
            Message::SearchQuery,
        )
        .id(INPUT_ID.clone())
        .padding(15)
        .size(16);

        let books: Element<_> = column(
            match &self.visible_books {
                Some(books) => {
                    books
                        .iter()
                        .enumerate()
                        .map(
                            |(i, res)|
                            view_search_result(
                                self.book_list[res.index],
                                res,
                                i+1
                            )
                        )
                        .collect()
                },
                None => {
                    self.book_list
                        .iter()
                        .enumerate()
                        .map(|(i, res)| view_book(res, i+1))
                        .collect()
                }
            }
        )
        .spacing(10)
        .into();

        let content = column![
            query_box,
            scrollable(
                container(
                    books
                )
                .width(Length::Fill)
                .padding(40)
            ),
        ]
        .spacing(10)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_items(Alignment::Center);

        container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(40)
        .center_x()
        .into()
    }
}

fn view_book<'a>(book_name: &'a str, i: usize) -> Element<'a, Message> {
    row!(
        text(
            format!(
                "{}. {}",
                i,
                book_name
            )
        ).size(20)
    ).into()
}

fn view_search_result<'a>(book_name: &'a str, search_result: &'a SearchResult, i: usize) -> Element<'a, Message> {
    let mut text_elements: Vec<Element<Message>> = vec!();
    text_elements.push(
        text(
            format!(
                "{}. ",
                i,
            )
        )
        .size(20)
        .into()
    );

    let mut i = 0usize;
    let mut segments: Vec<(Range<usize>, bool)> = vec!();

    search_result
        .ranges
        .iter()
        .for_each(|range| {
            if i < range.start {
                segments.push((i..range.start, false));
            }
            segments.push((range.clone(), true));
            i = range.end;
        });
    if i < book_name.len() {
        segments.push((i..book_name.len(), false));
    }

    text_elements.extend(
        segments
            .iter()
            .map(|(range, is_match)| {
                let text_label = text(String::from(&book_name[range.clone()])).size(20);
                if *is_match {
                    text_label.style(Color::from([1.0, 0.2, 0.2])).into()
                } else {
                    text_label.into()
                }
            })
    );

    row(text_elements).into()
}