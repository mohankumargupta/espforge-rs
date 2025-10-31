use serde::Deserialize;
use crate::Example;
use askama::Template;

#[derive(Clone, Debug, Template, Deserialize)]
#[template(path = "examples/hello_world/main.rs.askama")]
pub struct HelloWorldConfig;

impl Example for HelloWorldConfig {
    fn render(&self) -> Result<String, askama::Error> {
        Template::render(self)
    }
}
