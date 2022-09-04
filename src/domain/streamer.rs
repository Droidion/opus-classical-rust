/// Streaming service, e.g. Spotify, Tidal, or Apple Music.
pub struct Streamer {
    pub name: String,
    pub icon: String, // Logo filename, e.g. foo.webp
    pub link: String,
    pub prefix: String,
}