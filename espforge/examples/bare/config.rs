use serde::Deserialize;
use crate::Example;
use askama::Template;

#[derive(Deserialize)]
struct Config {}

#[derive(Template, Deserialize)]
#[template(path = "examples/blink/main.rs.askama")]
pub struct BareConfig {
    #[serde(flatten)]
    pub config: Config,
}

impl Example for BareConfig {
    fn render(&self) -> Result<String, askama::Error> {
        let template = BareConfig {};
        template.render();
    }
}
