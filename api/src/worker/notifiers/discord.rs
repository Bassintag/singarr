use std::{path::PathBuf, sync::Arc};

use anyhow::{anyhow, Result};
use reqwest::{
    multipart::{Form, Part},
    Client,
};
use serde::{Deserialize, Serialize};

use crate::{
    models::{event::Event, lyrics::Lyrics, notifier::NotifierContext},
    state::AppState,
};

#[derive(Default, Serialize)]
struct DiscordField {
    name: String,
    value: String,
    inline: bool,
}

#[derive(Default, Serialize)]
struct DiscordImage {
    url: String,
}

#[derive(Default, Serialize)]
struct DiscordAuthor {
    name: String,
}

#[derive(Default, Serialize)]
struct DiscordAttachment {
    id: usize,
    filename: String,
}

#[derive(Default, Serialize)]
struct DiscordEmbed {
    author: Option<DiscordAuthor>,
    title: Option<String>,
    description: Option<String>,
    url: Option<String>,
    timestamp: Option<String>,
    color: Option<i64>,
    image: Option<DiscordImage>,
    thumbnail: Option<DiscordImage>,
    fields: Option<Vec<DiscordField>>,
}

impl DiscordEmbed {
    fn from_lyrics(lyrics: &Lyrics) -> Self {
        Self {
            fields: Some(vec![
                DiscordField {
                    name: "Artist".into(),
                    value: lyrics.track.artist.name.clone(),
                    inline: false,
                },
                DiscordField {
                    name: "Album".into(),
                    value: lyrics.track.album.album.title.clone(),
                    inline: false,
                },
                DiscordField {
                    name: "Track".into(),
                    value: lyrics.track.title.clone(),
                    inline: false,
                },
                DiscordField {
                    name: "Synced".into(),
                    value: String::from(match lyrics.synced {
                        true => "Yes",
                        false => "No",
                    }),
                    inline: false,
                },
                DiscordField {
                    name: "Provider".into(),
                    value: match &lyrics.provider {
                        Some(provider) => provider.clone(),
                        None => "Manual".into(),
                    },
                    inline: false,
                },
                DiscordField {
                    name: "File".into(),
                    value: format!("`{}`", lyrics.file_path),
                    inline: false,
                },
            ]),
            ..Default::default()
        }
    }
}

struct Attachment {
    path: PathBuf,
    discord: DiscordAttachment,
}

struct DiscordMessageBuilder {
    embed: DiscordEmbed,
    attachments: Vec<Attachment>,
}

impl DiscordMessageBuilder {
    async fn from_lyrics(lyrics: &Lyrics) -> Result<Self> {
        let mut builder = DiscordMessageBuilder::from(DiscordEmbed::from_lyrics(&lyrics));
        if let Some(cover_path) = &lyrics.track.album.album.cover_path {
            builder = builder.image(PathBuf::from(cover_path))?;
        }
        if let Some(image_path) = &lyrics.track.album.artist.image_path {
            builder = builder.thumbnail(PathBuf::from(image_path))?;
        }
        Ok(builder)
    }

    fn title(mut self, title: String) -> Self {
        self.embed.title = Some(title);
        self
    }

    fn color(mut self, color: i64) -> Self {
        self.embed.color = Some(color);
        self
    }

    fn attach(&mut self, path: PathBuf) -> Result<String> {
        let id = self.attachments.len();
        let filename: String = path
            .file_name()
            .ok_or(anyhow!("Missing file_name"))?
            .to_str()
            .unwrap()
            .into();
        self.attachments.push(Attachment {
            path,
            discord: DiscordAttachment {
                id,
                filename: filename.clone(),
            },
        });
        Ok(filename)
    }

    fn image(mut self, path: PathBuf) -> Result<Self> {
        let filename = self.attach(path)?;
        self.embed.image = Some(DiscordImage {
            url: format!("attachment://{}", filename),
        });
        Ok(self)
    }

    fn thumbnail(mut self, path: PathBuf) -> Result<Self> {
        let filename = self.attach(path)?;
        self.embed.thumbnail = Some(DiscordImage {
            url: format!("attachment://{}", filename),
        });
        Ok(self)
    }

    async fn build(mut self, state: &Arc<AppState>) -> Result<Form> {
        self.embed.author = Some(DiscordAuthor {
            name: "Singarr".into(),
        });
        let mut attachments: Vec<DiscordAttachment> = Vec::new();
        let mut form = Form::new();
        for (i, attachment) in self.attachments.into_iter().enumerate() {
            let path = state.image_service.resolve_path(&attachment.path).await;
            let bytes = tokio::fs::read(&path).await?;
            let part = Part::bytes(bytes)
                .mime_str("image/webp")?
                .file_name(attachment.discord.filename.clone());
            form = form.part(format!("files[{}]", i), part);
            attachments.push(attachment.discord);
        }
        let message = DiscordMessage {
            embeds: Some(vec![self.embed]),
            attachments: Some(attachments),
            ..Default::default()
        };
        let payload_json = serde_json::to_string(&message)?;
        form = form.part(
            "payload_json",
            Part::text(payload_json).mime_str("application/json")?,
        );
        Ok(form)
    }
}

impl From<DiscordEmbed> for DiscordMessageBuilder {
    fn from(value: DiscordEmbed) -> Self {
        Self {
            embed: value,
            attachments: Vec::new(),
        }
    }
}

#[derive(Default, Serialize)]
struct DiscordMessage {
    content: Option<String>,
    embeds: Option<Vec<DiscordEmbed>>,
    attachments: Option<Vec<DiscordAttachment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscordParams {
    pub webhook_url: String,
}

pub async fn notify_discord(context: NotifierContext<DiscordParams>) -> Result<()> {
    let builder = match context.event.as_ref() {
        Event::LyricsCreated { lyrics } => DiscordMessageBuilder::from_lyrics(lyrics)
            .await?
            .title("Lyrics file imported".into())
            .color(0x00ff00),
        Event::LyricsDeleted { lyrics } => DiscordMessageBuilder::from_lyrics(lyrics)
            .await?
            .title("Lyrics file removed".into())
            .color(0xff0000),
        _ => return Ok(()),
    };

    let form = builder.build(&context.state).await?;
    Client::new()
        .post(context.params.webhook_url.as_str())
        .multipart(form)
        .send()
        .await?;

    Ok(())
}
