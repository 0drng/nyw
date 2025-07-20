use std::sync::LazyLock;

pub static SERVER_CONFIG: LazyLock<Settings> = LazyLock::new(|| Settings::build());

#[allow(non_snake_case)]
pub struct Settings {
    pub LOG_LEVEL: String,
}

impl Settings {
    fn build() -> Settings {
        Settings {
            LOG_LEVEL: std::env::var("LOG").unwrap_or(String::from("INFO")),
        }
    }
}
