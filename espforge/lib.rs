pub mod compile_command;
pub mod config;
pub mod core;
pub mod examples;

include!(concat!(env!("OUT_DIR"), "/examples_generated.rs"));

pub trait Example {
    /// Renders the Askama template for the example.
    fn render(&self) -> Result<String, askama::Error>;
}
