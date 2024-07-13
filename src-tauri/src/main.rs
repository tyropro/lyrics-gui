// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lazy_static::lazy_static;
use rspotify::{
    model::{AdditionalType, CurrentlyPlayingContext, FullTrack, PlayableItem},
    prelude::*,
    scopes, AuthCodeSpotify, Config, Credentials, OAuth,
};
use std::collections::HashSet;

// const LYRICS_API: &str = "https://lyrics.astrid.sh/api/search/";
const LYRICS_API: &str = "https://lyrics.vrsal.cc/api/search";

lazy_static! {
    static ref SPOTIFY: AuthCodeSpotify = {
        // get client credentials from `.env` file
        let creds: Credentials = Credentials::from_env().unwrap();

        let scopes: HashSet<String> = scopes!( // scopes for application
            "user-read-currently-playing",
            "user-read-playback-state",
            "user-read-recently-played",
            "user-read-private"
        );
        // get redirect uri from `.env` file and set scopes
        let oauth: OAuth = OAuth::from_env(scopes).unwrap();
        // set Config options (for token caching and refreshing)
        let config: Config = Config {
            token_cached: true,
            token_refreshing: true,
            ..Default::default()
        };

        // create Spotify client instance with config
        let spotify: AuthCodeSpotify = AuthCodeSpotify::with_config(creds, oauth, config);

        spotify
    };
}

#[tauri::command]
async fn get_spotify_song() -> Result<String, String> {
    // get currently playing track
    let currently_playing: CurrentlyPlayingContext = SPOTIFY
        .current_playing(None, Some(vec![&AdditionalType::Track]))
        .await
        .unwrap()
        .unwrap();

    // get track from currently playing track
    let content: PlayableItem = currently_playing.item.unwrap();
    let track: FullTrack = match content {
        PlayableItem::Track(track) => track,
        _ => panic!("Not a track"),
    };

    // get track name and artist name
    let track_name: &String = &track.name;
    let artist_name: &String = &track.artists[0].name;
    let query: String = format!("{} {}", track_name, artist_name);

    Ok(query)
}

#[tauri::command]
fn get_lyrics(query: &str) -> Result<String, String> {
    // get currently playing track
    let lyrics_url: String = url::Url::parse_with_params(LYRICS_API, [("q", query)])
        .unwrap()
        .to_string(); // parse the ur;
    let request = ureq::get(&lyrics_url); // create the request
    let response = request.call(); // fetch the response

    if response.is_ok() {
        let resp = response.unwrap();
        let json_reponse: serde_json::Value = resp.into_json().unwrap(); // parse the response as json

        let lyrics: String = json_reponse[0]["plainLyrics"]
            .as_str()
            .unwrap()
            .trim()
            .to_string(); // get the lyrics from the json response

        Ok(lyrics)
    } else {
        Err("Error occured when getting lyrics".to_string())
    }
}

#[tauri::command]
fn get_synced_lyrics(query: &str) -> Result<String, String> {
    let synced_lyrics_url: String = url::Url::parse_with_params(LYRICS_API, [("q", query)])
        .unwrap()
        .to_string();
    let request = ureq::get(&synced_lyrics_url);
    let response = request.call();

    if response.is_ok() {
        let resp = response.unwrap();
        let json_reponse: serde_json::Value = resp.into_json().unwrap();
        let lyrics: String = json_reponse[0]["syncedLyrics"]
            .as_str()
            .unwrap()
            .trim()
            .to_string();
        Ok(lyrics)
    } else {
        Err("Error occured when getting synced lyrics".to_string())
    }
}

#[tokio::main]
async fn main() {
    let url: String = SPOTIFY.get_authorize_url(false).unwrap();
    SPOTIFY.prompt_for_token(&url).await.unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_spotify_song, get_lyrics])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
