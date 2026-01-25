# Singarr

Singarr is a companion application to [Lidarr](https://lidarr.audio/). It manages and downloads **lyrics** for the tracks that Lidarr indexes.

> ⚠️ Singarr **does not scan your disk** to detect tracks. It only works with tracks already known by Lidarr.

---

## Features

- Automatically download lyrics for your music library indexed in Lidarr.
- Web interface for browsing, editing, and managing lyrics.
- Lightweight and easy to deploy (Docker-ready).

---

## Screenshot

![Singarr](/screenshot/screenshot-1.png?raw=true "Singarr")

---

## Getting Started

### Docker

```bash
docker run -d \
  --name singarr \
  -p 8080:80 \
  -v /path/to/config:/config \
  -v /path/to/media:/media \
  ghcr.io/Bassintag/singarr:latest
```

### Docker Compose

```bash
singarr:
    image: ghcr.io/bassintag/singarr:lastest
    container_name: singarr
    ports:
        - 8080:80
    volumes:
        - /path/to/config:/config
        - /path/to/media:/media
    restart: unless-stopped
```
