use serde::Deserialize;
use crate::Example;
use askama::Template;

#[derive(Deserialize)]
struct Config;

#[derive(Template, Deserialize)]
#[template(path = "examples/hello_world/main.rs.askama")]
pub struct HelloWorldConfig {
    #[serde(flatten)]
    pub config: Config,
}

impl Example for HelloWorldConfig {
    fn render(&self) -> Result<String, askama::Error> {
        let template = BlinkConfig{};
        template.render();
    }
}
