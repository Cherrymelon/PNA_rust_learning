use clap::Command;
use clap::Parser;
use clap::Subcommand;

use kvs::KvStore;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Get {
        /// lists test values
        #[arg(short, long)]
        key: String,
    },
    Set {
        #[arg(short, long)]
        key: String,
        #[arg(short, long)]
        value: String,
    },
    Remove {
        #[arg(short, long)]
        key: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut store = KvStore::new();
    match &cli.command {
        Some(Commands::Get { key }) => {
            println!("{:?}", store.get(key.to_string()))
        }
        Some(Commands::Remove { key }) => {
            println!("{:?}", store.remove(key.to_string()))
        }
        Some(Commands::Set { key, value }) => {
            println!("{:?}", store.set(key.to_string(), value.to_string()))
        }
        None => {}
    }
}
