use clap::{Args, Parser, Subcommand};
use std::thread;
use std::time;
use zookeeper::{WatchedEvent, Watcher, ZkError, ZooKeeper};

struct LoggingWatcher;

impl Watcher for LoggingWatcher {
    fn handle(&self, _e: WatchedEvent) {}
}

/// try to connect to the server by the given address
fn connect_zk(addr: &str) -> Result<ZooKeeper, ZkError> {
    let mut retry = 0;
    loop {
        match ZooKeeper::connect(addr, time::Duration::from_secs(5), LoggingWatcher) {
            Ok(zk_cli) => {
                return Ok(zk_cli);
            }
            Err(e) => {
                println!("Error connecting to ZooKeeper: {}", e);
                retry += 1;
                if retry >= 10 {
                    return Err(e);
                }
                thread::sleep(time::Duration::from_millis(10));
            }
        }
    }
}
#[derive(Debug, Parser)]
#[command(author="yangsen.zk",version="0.0.1",about="None interactive zookeeper client",long_about=None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(long)]
    address: String,
    #[command(subcommand)]
    command: Option<SubCommands>,
}
#[derive(Subcommand, Debug, Clone)]
enum SubCommands {
    #[command(name = "create")]
    Create(Create),
    #[command(name = "set")]
    Set {
        #[arg(long)]
        path: String,
        #[arg(long)]
        value: Option<String>,
        #[arg(long, default_value_t = 0)]
        random_value_size: usize,
    },
    #[command(name = "exists")]
    Exists {
        #[arg(long)]
        path: String,
    },
    #[command(name = "delete")]
    Delete {
        #[arg(long)]
        path: String,
    },
    #[command(name = "deleteall")]
    DeleteAll {
        #[arg(long)]
        path: String,
    },
}
#[derive(Debug, Args)]
#[command(name = "create"]
struct Create {
    #[arg(long)]
    path: String,
    #[arg(long)]
    value: Option<String>,
    #[arg(long, default_value_t = 0)]
    random_value_size: usize,
}
fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli.command);
    match cli.command.unwrap() {
        SubCommands::Create {
            path,
            value,
            random_value_size,
        } => {
            println!("{:?}", path);
        }
        SubCommands::Set {
            path,
            value,
            random_value_size,
        } => {
            println!("{}", path);
        }
        SubCommands::Delete { path } => {
            println!("{}", path);
        }
        SubCommands::DeleteAll { path } => {
            println!("{}", path);
        }
        SubCommands::Exists { path } => {
            println!("{}", path);
        }
    };
}
