use crate::{models::track::Track, worker::provider::SearchResult};

pub fn score_result(track: &Track, result: &SearchResult) -> f64 {
    let score_track_name = score_strings(&track.title, &result.track_name);
    let score_artist_name = score_strings(&track.artist.name, &result.artist_name);
    let score_album_title = score_strings(&track.album.album.title, &result.album_title);
    let score_duration = match result.duration_ms {
        Some(duration_ms) => score_durations(track.duration_ms, duration_ms),
        None => 0.5,
    };

    score_duration * 0.4
        + score_track_name * 0.2
        + score_artist_name * 0.2
        + score_album_title * 0.2
}

fn score_strings(a: &String, b: &String) -> f64 {
    strsim::jaro_winkler(a, b)
}

fn score_durations(a: i64, b: i64) -> f64 {
    let diff = (a - b).abs();
    (1.0 - (diff as f64 / 10_000.0)).clamp(0.0, 1.0)
}
