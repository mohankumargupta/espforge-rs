use serde::Deserialize;
use crate::Example;
use askama::Template;

#[derive(Clone, Debug, Deserialize)]
struct Config {}

#[derive(Clone, Debug, Template, Deserialize)]
#[template(path = "examples/bare/main.rs.askama")]
pub struct BareConfig {
    #[serde(flatten)]
    pub config: Config,
}

impl Example for BareConfig {
    fn render(&self) -> Result<String, askama::Error> {
        Template::render(self)
    }
}
