use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "RusToDo",
    version = "1.0",
    author = "MasaHero",
    about = "CLI Based TODOList Application"
)]
pub struct Args {
    #[arg(short, long, help = "List up TODO's (Default)")]
    pub list: bool,

    #[arg(short = 'A', long, help = "Add TODO")]
    pub add: Option<String>,

    #[arg(short = 'U', long, help = "Update TODO")]
    pub update: Option<u32>,

    #[arg(short = 'D', long, help = "Delete TODO")]
    pub delete: Option<u32>,

    #[arg(short = 'S', long, help = "Show Detail TODO")]
    pub show: Option<u32>,

    #[arg(short, long, help = "Set TODO name")]
    pub name: Option<String>,

    #[arg(short, long, help = "Set TODO text")]
    pub text: Option<String>,

    #[arg(short = 'e', long, help = "Set due date (format: YYYY/MM/DD HH:MM:SS)")]
    pub due_date: Option<String>,

    #[arg(short, long, help = "Set priority (Low, Medium, High)")]
    pub priority: Option<String>,

    #[arg(
        short = 's',
        long,
        help = "Set status (Pending, InProgress, Completed)"
    )]
    pub status: Option<String>,
}
