use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author,version,about,long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[derive(Parser)]
#[command(version, about, long_about = "None")]
struct Cli {
    #[arg(short, long)]
    name: Option<String>,
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short,long,action=clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Test {
        #[arg(short, long)]
        list: bool,
    },
}

#[derive(Subcommand)]
enum HttpCommand {
    Get {
        #[arg(short, long)]
        url: String,
    },
    Post {
        #[arg(short, long)]
        url: String,
    },
}

/**
 * usage
 *  cargo run -- -dd  -n myname test
 */
fn main() {
    let cli = Cli::parse();

    if let Some(name) = cli.name.as_deref() {
        println!("value for name : {}", name);
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("value for config : {}", config_path.display());
    }
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("print testing list ...");
            } else {
                println!("no list");
            }
        }
        None => {
            println!("no command")
        }
    }

    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        _ => println!("Debug mode unknown {}", cli.debug),
    }
}
