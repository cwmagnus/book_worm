use lazy_static::lazy_static;
use regex::Regex;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::utils::Colour;
use urlencoding;
use crate::book::BookVolumeCollection;

const GOOGLE_API_KEY: &str = "AIzaSyBZWD8GhUNKmq_HUc1l5mb1LPsJF3QDOoc";

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\{(?P<book>[^']+)\}").unwrap();
        }

        for cap in RE.captures_iter(&msg.content) {
            let request_url = format!("https://www.googleapis.com/books/v1/volumes?q={search}&key={key}", search = urlencoding::encode(&cap["book"]), key = GOOGLE_API_KEY);
            let response = reqwest::get(&request_url).await.unwrap();
            let response_text = &response.text().await.unwrap();
            let volume_collection = serde_json::from_str::<BookVolumeCollection>(&response_text).unwrap();

            for volume in volume_collection.items {

                if volume.volume_info.authors.is_empty() || volume.volume_info.description.is_empty() || volume.volume_info.title.is_empty() {
                    continue;
                }

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

                return;
            }
        }
    }
}
