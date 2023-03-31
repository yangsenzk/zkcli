use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version = "0.0.1", about = "None interactive zookeeper client", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(long)]
    pub address: String,
    #[command(subcommand)]
    pub command: Option<SubCommands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum SubCommands {
    #[command(name = "create")]
    Create(Create),
    #[command(name = "get")]
    Get(Get),
    #[command(name = "set")]
    Set(Set),
    #[command(name = "exists")]
    Exists(Exists),
    #[command(name = "delete")]
    Delete(Delete),
    #[command(name = "deleteall")]
    DeleteAll(DeleteAll),
}

#[derive(Debug, Clone, Args)]
pub struct Create {
    #[arg(long)]
    pub path: String,
    #[arg(long)]
    pub value: Option<String>,
    #[arg(long, default_value_t = 0)]
    pub random_size: usize,
}

#[derive(Debug, Clone, Args)]
pub struct Set {
    #[arg(long)]
    pub path: String,
    #[arg(long)]
    pub value: Option<String>,
    #[arg(long, default_value_t = 0)]
    pub random_size: usize,
}

#[derive(Debug, Clone, Args)]
pub struct Exists {
    #[arg(long)]
    pub path: String,
}

#[derive(Debug, Clone, Args)]
pub struct Delete {
    #[arg(long)]
    pub path: String,
}

#[derive(Debug, Clone, Args)]
pub struct DeleteAll {
    #[arg(long)]
    pub path: String,
}

#[derive(Debug, Clone, Args)]
pub struct Get {
    #[arg(long)]
    pub path: String,
}
