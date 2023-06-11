use std::{collections::HashMap, str::FromStr};

use args::{KyuunArgs, KyuunCommand};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input};
use miette::IntoDiagnostic;
use owo_colors::OwoColorize;
use warp::{hyper::Uri, Filter};

use crate::{
    api::{Playlist, Token, Tracks},
    config::Config,
};

mod api;
mod args;
mod config;
mod log;

#[tokio::main]
async fn main() -> miette::Result<()> {
    let args = KyuunArgs::parse();
    match args.command {
        Some(KyuunCommand::Setup(_)) => setup().await,
        _ => run(args.id).await,
    }
}

async fn setup() -> miette::Result<()> {
    let client_id: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Spotify client ID")
        .interact_text()
        .into_diagnostic()?;
    let client_secret: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Spotify client secret")
        .interact_text()
        .into_diagnostic()?;
    let client_id_clone = client_id.clone();
    let mut cfg = Config {
        client_id: client_id.clone(),
        client_secret: client_secret.clone(),
        ..Default::default()
    };
    tokio::task::spawn(async move {
        let home = warp::path::end()
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| if let Some(code) = p.get("code") {
            warp::reply::html(
            format!("<title>kyuun</title><link rel=\"stylesheet\" href=\"https://unpkg.com/sakura.css/css/sakura-vader.css\"><script>function copy(){{navigator.clipboard.writeText(document.querySelector(\"pre\").innerText);alert(\"Copied\");}}</script><h1>kyuun</h1><div><pre>{code}</pre><button onclick=\"copy()\">Copy</button></div><h4>Steps:</h4><ul><li>Copy the code above.</li><li>Paste it when prompted after running <code>kyuun setup</code>.</li></ul>"),
        )} else {
            warp::reply::html("<title>kyuun</title><link rel=\"stylesheet\" href=\"https://unpkg.com/sakura.css/css/sakura-vader.css\"/><h1>kyuun</h1><div>bun bun bun</div>".to_owned())
        });

        let url = format!("https://accounts.spotify.com/authorize?response_type=code&client_id={client_id_clone}&redirect_uri=http://localhost:1410&scope=playlist-read-private%20playlist-read-collaborative");
        let auth = warp::path("auth").map(move || warp::redirect(Uri::from_str(&url).unwrap()));
        warp::serve(home.or(auth)).run(([0, 0, 0, 0], 1410)).await;
    });
    let auth_url = "http://localhost:1410/auth";
    open::that(auth_url).into_diagnostic()?;
    info!(
        "Opened {} in your browser. {}",
        auth_url.bright_blue(),
        "(if it didn't open, copy and paste the URL into your browser)"
            .bright_black()
            .italic()
    );
    let code: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("kyuun code")
        .interact_text()
        .into_diagnostic()?;
    let token = reqwest::Client::new()
        .post("https://accounts.spotify.com/api/token")
        .basic_auth(client_id.clone(), Some(client_secret))
        .form(&[
            ("grant_type", "authorization_code"),
            ("redirect_uri", "http://localhost:1410"),
            ("code", &code),
        ])
        .send()
        .await
        .into_diagnostic()?
        .json::<Token>()
        .await
        .into_diagnostic()?;

    cfg = Config {
        authorization_code: code,
        access_token: token.access_token,
        refresh_token: token.refresh_token.unwrap(),
        expires_in: chrono::Utc::now().timestamp() + token.expires_in,
        ..cfg
    };

    confy::store("kyuun", "kyuun", &cfg).into_diagnostic()?;

    Ok(())
}

async fn run(id_: Option<String>) -> miette::Result<()> {
    let mut cfg = confy::load::<Config>("kyuun", "kyuun").into_diagnostic()?;

    // refresh token if expired
    if chrono::Utc::now().timestamp() > cfg.expires_in {
        let token = reqwest::Client::new()
            .post("https://accounts.spotify.com/api/token")
            .basic_auth(cfg.client_id.clone(), Some(cfg.client_secret.clone()))
            .form(&[
                ("grant_type", "refresh_token"),
                ("refresh_token", &cfg.refresh_token),
            ])
            .send()
            .await
            .into_diagnostic()?
            .json::<Token>()
            .await
            .into_diagnostic()?;
        cfg = Config {
            access_token: token.access_token,
            expires_in: chrono::Utc::now().timestamp() + token.expires_in,
            ..cfg
        };
        confy::store("kyuun", "kyuun", &cfg).into_diagnostic()?;
    }

    let playlist_id = match id_ {
        Some(id) => id,
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Playlist ID")
            .interact_text()
            .into_diagnostic()?,
    };

    let client = reqwest::Client::new();
    let token = cfg.access_token;

    let mut artists: HashMap<String, u32> = HashMap::new();
    let playlist = client
        .get(format!(
            "https://api.spotify.com/v1/playlists/{playlist_id}"
        ))
        .bearer_auth(token.clone())
        .send()
        .await
        .into_diagnostic()?
        .json::<Playlist>()
        .await
        .into_diagnostic()?;
    println!("{}\nby {}", playlist.name.bold(), playlist.owner.id.bold());

    let mut tracks = playlist.tracks;

    loop {
        for track in tracks.items.into_iter().map(|item| item.track) {
            for artist in track.artists {
                let name = artist.name;
                artists.insert(name.clone(), artists.get(&name).unwrap_or(&0) + 1);
            }
        }

        if let Some(next) = tracks.next.clone() {
            tracks = client
                .get(next)
                .bearer_auth(token.clone())
                .send()
                .await
                .into_diagnostic()?
                .json::<Tracks>()
                .await
                .into_diagnostic()?;
        } else {
            break;
        }
    }

    let mut sorted_artists = artists.iter().collect::<Vec<(&String, &u32)>>();
    sorted_artists.sort(); // sort by artist name alphabetically
    sorted_artists.sort_by(|k, v| v.1.cmp(k.1)); // then sort by count

    println!("{} Songs, {} Artists\n", tracks.total, artists.len());
    for (artist, count) in sorted_artists {
        info!(
            "{artist}: {} ({})",
            count.bold(),
            format!("{:.02}%", (*count as f32 / tracks.total as f32 * 100.0))
                .bright_magenta()
                .italic()
        );
    }

    Ok(())
}
