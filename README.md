# fuzzyserve

HTTP server that redirects fuzzy file queries to actual media files.

## Features

- Fuzzy matching of filenames using [skim](https://github.com/skim-rs/fuzzy-matcher) algorithm
- File system watching with automatic reloading
- Built-in directory listing
- Graceful shutdown

## Installation

```bash
cargo install --path .
```

Or build from source:

```bash
cargo build --release
```

## Usage

```bash
fuzzyserve --media-root /path/to/media --port 8080 --addr 0.0.0.0
```

### Options

| Option         | Short | Default   | Description                    |
| -------------- | ----- | --------- | ------------------------------ |
| `--media-root` | `-m`  | `.`       | Root directory for media files |
| `--port`       | `-p`  | `7666`    | HTTP port                      |
| `--addr`       | `-a`  | `0.0.0.0` | Bind address                   |

## Endpoints

### `GET /get/{query}`

Redirects (HTTP 303) to the best matching file.

```bash
# Find Simpsons S37E05
curl -L http://localhost:7666/get/simpsons-s37e05

# Find Home Alone 2
curl -L http://localhost:7666/get/home-alone-2
```

### `GET /files/`

Directory listing and direct file access.

```bash
# Browse files
curl http://localhost:7666/files/

# Download specific file
curl http://localhost:7666/files/Movies/movie.mkv
```

## Examples

```bash
# Serve current directory
fuzzyserve

# Serve media library
fuzzyserve -m /srv/media -p 8080

# Use with VLC
vlc http://localhost:7666/get/simpsons-s37e12
```
