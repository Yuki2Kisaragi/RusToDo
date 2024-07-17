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

    #[arg(short, long, value_name = "TITLE", help = "Add TODO with title")]
    pub add: Option<String>,

    #[arg(short, long, value_name = "ID", help = "Update TODO by ID")]
    pub update: Option<u32>,

    #[arg(short, long, value_name = "ID", help = "Delete TODO by ID")]
    pub delete: Option<u32>,

    #[arg(short, long, value_name = "ID", help = "Show detail of TODO by ID")]
    pub show: Option<u32>,

    #[arg(short, long, value_name = "NAME", help = "Update TODO Name")]
    pub name: Option<String>,

    #[arg(short, long, value_name = "TEXT", help = "TEXT for add or update")]
    pub text: Option<String>,

    #[arg(
        short,
        long,
        value_name = "DATE",
        help = "Due date for add or update (format: YYYY/MM/DD HH:MM:SS)"
    )]
    pub due_date: Option<String>,

    #[arg(
        short,
        long,
        value_name = "PRIORITY",
        help = "Priority for add or update (Low, Medium, High)"
    )]
    pub priority: Option<String>,

    #[arg(
        long,
        value_name = "STATUS",
        help = "Status for update (Pending, InProgress, Completed)"
    )]
    pub status: Option<String>,
}
