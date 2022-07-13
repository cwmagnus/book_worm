use lazy_static::lazy_static;
use regex::Regex;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::utils::Colour;
use urlencoding;
use crate::book::{BookVolume, BookVolumeCollection};

const GOOGLE_API_KEY: &str = "AIzaSyBZWD8GhUNKmq_HUc1l5mb1LPsJF3QDOoc";

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\{(?P<book>.*?)\}").unwrap();
        }

        for cap in RE.captures_iter(&msg.content) {
            let request_url = format!("https://www.googleapis.com/books/v1/volumes?q={}&key={}", urlencoding::encode(&cap["book"]), GOOGLE_API_KEY);

            let response = reqwest::get(&request_url).await;
            if let Err(error) = response  {
                println!("Error getting Google Books response: {}", &error.to_string());
                continue;
            }

            let response = response.unwrap();
            let response_text = &response.text().await.unwrap();

            let volume_collection = serde_json::from_str::<BookVolumeCollection>(&response_text);
            if let Err(error) = volume_collection  {
                println!("Error parsing json response: {}", &error.to_string());
                continue;
            }

            let volume_collection = volume_collection.unwrap();
            if volume_collection.items.is_empty() {
                let msg = msg
                    .channel_id
                    .send_message(&_ctx.http, |m| {
                        m.embed(|e| {
                            e.color(Colour::from_rgb(121, 21, 81));
                            e.title("\"You may ask questions which I shall not choose to answer.\"\n(No books found)")
                        })
                    }).await;

                if let Err(why) = msg {
                    println!("Error sending message: {:?}", why);
                }

                continue;
            }

            if let Some(volume) = find_best_book(volume_collection.items) {
                let msg = msg
                    .channel_id
                    .send_message(&_ctx.http, |m| {
                        m.embed(|e| {
                            e.color(Colour::from_rgb(121, 21, 81));

                            let mut title = volume.volume_info.title.to_string();
                            let subtitle = &volume.volume_info.subtitle;
                            if !subtitle.is_empty() {
                                title += &*format!(": {}", &subtitle);
                            }

                            let author_label = if volume.volume_info.authors.len() > 1 {
                                "Authors"
                            } else {
                                "Author"
                            };

                            let mut authors = String::new();
                            for author in &volume.volume_info.authors {
                                let comma = if !authors.is_empty() {
                                    ", "
                                } else {
                                    ""
                                };
                                authors += &*format!("{}{}", comma, &author);
                            }

                            e.title(title)
                                .url(&volume.volume_info.info_link)
                                .description(format!("**{}**\n{}\n\n**Description**\n{}",
                                                     author_label,
                                                     &authors,
                                                     &volume.volume_info.description))
                                .image(&volume.volume_info.image_links.thumbnail)
                        })
                    }).await;

                if let Err(why) = msg {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }
}

fn find_best_book(books: Vec<BookVolume>) -> Option<BookVolume> {
    for book in books {
        if book.volume_info.authors.is_empty() || book.volume_info.description.is_empty() || book.volume_info.title.is_empty() {
            continue;
        }

        return Some(book);
    }

    None
}
