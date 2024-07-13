# Lyrics GUI

This is a simple GUI that gets lyrics from a search query.

## Functionality

- Search for lyrics by song, artist, or album
- Fill the search box with the currently playing song on Spotify
- Open a browser tab with Ultimate Guitar tabs for the song


## DISCLAIMER
The Spotify API implementation is not fit for GUI use. I am yet to reverse engineer the process the Spotify Wrapper (RSpotify) uses to get the Spotify token. You will instead have to get the token from my [Spotify CLI](https://github.com/tyropro/rs-spotify-lyrics) and copy the `.spotify-token-cache.json` file to the root directory of the app.


## Credits

- [RSpotify](https://github.com/ramsayleung/rspotify): A full-featured Spotify API wrapper for Rust
- [Astridlol's Lyrics](https://github.com/astridlol/lyrics): A really easy and free to use API for getting unsynced song lyrics without an API key.

## License

This project is licensed under the [MIT License](LICENSE).