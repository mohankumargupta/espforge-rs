use serde::Deserialize;
use crate::Example;
use askama::Template;

#[derive(Clone, Debug, Template, Deserialize)]
#[template(path = "templates/blink/main.rs.askama")]
pub struct BlinkConfig {
    pub blink_rate_ms: u64,
}

impl Example for BlinkConfig {
    fn render(&self) -> Result<String, askama::Error> {
        Template::render(self)
    }
}



