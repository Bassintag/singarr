use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version)]
pub struct AppArgs {
    #[arg(long, env = "DATABASE_URL", default_value = "sqlite::memory:")]
    pub database_url: String,

    #[arg(long, env = "SETTINGS_PATH", default_value = "./data/settings.json")]
    pub settings_path: String,
}
