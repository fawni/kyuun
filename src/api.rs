use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Token {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Playlist {
    pub name: String,
    pub owner: PlaylistOwner,
    pub tracks: Tracks,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PlaylistOwner {
    pub id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Tracks {
    pub items: Vec<Item>,
    pub next: Option<String>,
    pub total: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Item {
    pub track: Track,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Track {
    pub artists: Vec<Artist>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Artist {
    pub name: String,
}
