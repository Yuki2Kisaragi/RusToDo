use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "RusToDo",
    version = "0.1",
    author = "MasaHero <yourmail@example.com>",
    about = "CLI上で動かせるTODOアプリです。"
)]
pub struct Args {
    #[arg(short, long, help = "List up TODO's (Default)")]
    pub list: bool,

    #[arg(short, long, help = "Add TODO")]
    pub add: bool,

    #[arg(short, long, help = "Update TODO")]
    pub update: bool,

    #[arg(short, long, help = "Delete TODO")]
    pub delete: bool,

    #[arg(short, long, help = "Show Detail TODO")]
    pub show: bool,
}
