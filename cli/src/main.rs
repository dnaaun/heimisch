mod auth;
mod consts;
mod db;
mod error;
mod utils;

use clap::{Parser, Subcommand};
use db::get_migrated_pool;
use error::Result;

#[derive(Parser)]
#[command(name = "lh")]
#[command(about = "CLI for authentication", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Handle authentication operations
    Auth {
        #[command(subcommand)]
        operation: AuthOperation,
    },
}

#[derive(Subcommand)]
enum AuthOperation {
    Login {
        /// Login via web login. Required for now because we have no other ways of logging in.
        #[arg(long, required = true)]
        web: bool,
    },
    Logout,
    // TODO: A command that should not be commited to prod (or whatever)
    // Playground,
}

async fn actual_main() -> Result<()> {
    let cli = Cli::parse();
    let pool = get_migrated_pool().await?;

    let result = match cli.command {
        Command::Auth { operation } => match operation {
            AuthOperation::Login { web: _ } => auth::login(&pool).await,
            AuthOperation::Logout => auth::logout(&pool).await,
            // AuthOperation::Playground => playground(&pool).await,
        },
    };

    if let Err(err) = &result {
        println!("{err}");
        println!("{}", err.backtrace());
    }

    result
}

#[tokio::main]
async fn main() -> std::result::Result<(), ()> {
    actual_main().await.map_err(|_| ())
}
