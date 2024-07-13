import { useState } from "react";
// import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
	const [lyrics, setLyrics] = useState<string>("");
	const [query, setQuery] = useState("");

	async function get_lyrics() {
		setLyrics("Working...");
		try {
			setLyrics(await invoke("get_lyrics", { query }));
		} catch (e) {
			setLyrics(e as string);
		}
	}

	async function get_song_from_spotify() {
		setQuery(await invoke("get_spotify_song", {}));
	}

	return (
		<div className="container">
			<div className="input">
				<form
					className="query-form"
					onSubmit={(e) => {
						e.preventDefault();
						get_lyrics();
					}}
				>
					<input
						id="query-input"
						onChange={(e) => setQuery(e.currentTarget.value)}
						placeholder="Enter a song..."
						className="query-item"
						value={query}
					/>
					<button
						className="btn query-item"
						id="get-lyrics-btn"
						type="submit"
					>
						Get
					</button>
				</form>

				<button
					id="get-spotify-song-btn"
					className="btn query-item"
					onClick={() => get_song_from_spotify()}
				>
					Spotify
				</button>

				<a
					className="btn query-item"
					id="tab-btn"
					target="_blank"
					href={"https://www.ultimate-guitar.com/search.php?title=" + query + "&page=1&type=200"}
				>
					TAB
				</a>
			</div>

			<p id="lyrics">{lyrics}</p>
		</div>
	);
}

export default App;
