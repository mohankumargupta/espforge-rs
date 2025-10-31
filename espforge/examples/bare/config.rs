use serde::Deserialize;
use crate::Example;
use askama::Template;

#[derive(Clone, Debug, Template, Deserialize)]
#[template(path = "examples/bare/main.rs.askama")]
pub struct BareConfig;

impl Example for BareConfig {
    fn render(&self) -> Result<String, askama::Error> {
        Template::render(self)
    }
}
