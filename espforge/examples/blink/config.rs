use serde::Deserialize;
use crate::Example;
use askama::Template;

#[derive(Deserialize)]
struct Config {
    pub blink_rate_ms: u64,
}

#[derive(Template, Deserialize)]
#[template(path = "examples/blink/main.rs.askama")]
pub struct BlinkConfig {
    #[serde(flatten)]
    pub config: Config,
}

impl Example for BlinkConfig {
    fn render(&self) -> Result<String, askama::Error> {
        let template = BlinkConfig {
            blink_rate_ms: self.config.blink_rate_ms,
        };
        template.render()
    }
}



