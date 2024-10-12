pub mod webhook_desc_to_rust;

use std::path::PathBuf;

use clap::Parser;
use webhook_desc_to_rust::webhook_desc_to_rust;

#[derive(Parser)]
enum App {
    WebhookDescToRust { json_file: PathBuf },
}

fn main() -> anyhow::Result<()> {
    let cli = App::parse();
    match cli {
        App::WebhookDescToRust { json_file } => webhook_desc_to_rust(&json_file),
    }
}
