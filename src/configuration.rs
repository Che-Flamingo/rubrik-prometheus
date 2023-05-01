#[derive(serde::Deserialize)]
pub struct Settings {
    pub token: String,
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(
            config::File::new("configuration.yaml", config::FileFormat::Yaml)
        )
        .build()?;
    settings.try_deserialize::<Settings>()
}