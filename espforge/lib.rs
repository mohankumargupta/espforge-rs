pub mod compile_command;
pub mod config;
pub mod core;

pub trait Example {
    /// Renders the Askama template for the example.
    fn render(&self) -> Result<String, askama::Error>;
}
