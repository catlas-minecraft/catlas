mod command;

use clap::{Parser, Subcommand};
use command::render;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CatlasCli {
    #[command(subcommand)]
    pub subcommand: SubCommands,
}

#[derive(Subcommand, Debug)]
pub enum SubCommands {
    Render(render::RenderOption),
}

pub fn cli() {
    let args = CatlasCli::parse();

    match args.subcommand {
        SubCommands::Render(option) => {
            render::render(option);
        }
    }
}
