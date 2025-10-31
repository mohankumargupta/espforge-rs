use serde::Deserialize;
use crate::Example;
use askama::Template;

#[derive(Clone, Debug, Deserialize)]
struct Config {
    pub blink_rate_ms: u64,
}

#[derive(Clone, Debug, Template, Deserialize)]
#[template(path = "examples/blink/main.rs.askama")]
pub struct BlinkConfig {
    #[serde(flatten)]
    config: Config,
}

impl Example for BlinkConfig {
    fn render(&self) -> Result<String, askama::Error> {
        Template::render(self)
    }
}



