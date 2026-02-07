use clap::{Parser, Subcommand, Args as ClapArgs};


#[derive(Parser, Debug)]
#[command(name = "crypt-files")]
#[command(about = "Local encrypted file manager", long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub verbose: bool,

    #[arg(short, long)]
    pub force: bool,

    #[arg(long)]
    pub debug: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new encrypted storage
    Init(InitArgs),

    /// Get secrets for an application
    Get(GetArgs),

    /// Exit the program
    Exit,
}

#[derive(ClapArgs, Debug)]
pub struct InitArgs {
    #[arg(long, default_value = ".")]
    pub path: String,

    #[arg(long, default_value = "storage")]
    pub name: String,

    #[arg(long, required = true)]
    pub password: String,

    #[arg(long)]
    pub overwrite: bool,
}

#[derive(ClapArgs, Debug)]
pub struct GetArgs {
    #[arg(long)]
    pub app: Option<String>,
}

