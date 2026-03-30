# dow

A simple program to download music from Spotify.

## Installation

To install the `dow` program, follow these steps:

1. Clone the repository: `git clone https://github.com/your-username/dow.git`
2. Navigate to the project directory: `cd dow`
3. Build the project: `cargo build --release`
4. Run the binary: `./target/degub/dow.exe`

## Usage

To use the `dow` program, run the following command:

```bash
dow --playlist <playlist-url> --output <output-path>
dow --p <playlist-url> --o <output-path>
```

- `--playlist` or `-p` is the URL or code of the Spotify playlist you want to download.
- `--output` or `-p`is the path where you want to save the downloaded music.

For example:

```bash
dow --playlist 3gh1cUVq6xw082KmdIghwL --output /path/to/save/music
```

<!-- ## Adding a New Song

To add a new song to the playlist, follow these steps:

1. Find the Spotify URL of the song you want to add. You can usually find this by right-clicking on the song and selecting "Copy Spotify URL".
2. Run the following command:

   ```bash
   dow add --playlist <playlist-url> --song <song-url>
   ```

   - `--playlist` is the URL or code of the Spotify playlist you want to add the song to.
   - `--song` is the URL of the song you want to add.

   For example:

   ```bash
   dow add --playlist 3gh1cUVq6xw082KmdIghwL --song https://open.spotify.com/track/0a0nr0T7UuIw7oX7C9BZx6
   ``` -->

## Contributing

If you want to contribute to this project, feel free to open a pull request or submit an issue.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.