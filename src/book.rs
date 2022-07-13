use serde::*;

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct BookVolumeCollection {
    pub items: Vec<BookVolume>
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct BookVolume {
    #[serde(rename = "volumeInfo")]
    pub volume_info: VolumeInfo
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct VolumeInfo {
    pub title: String,
    pub subtitle: String,
    pub authors: Vec<String>,
    pub description: String,
    #[serde(rename = "averageRating")]
    pub average_rating: f32,
    #[serde(rename = "ratingsCount")]
    pub ratings_count: f32,
    #[serde(rename = "imageLinks")]
    pub image_links: ImageLinks,
    #[serde(rename = "infoLink")]
    pub info_link: String
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct ImageLinks {
    #[serde(rename = "smallThumbnail")]
    pub small_thumbnail: String,
    pub thumbnail: String
}
