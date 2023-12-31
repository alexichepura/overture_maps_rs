use clap::{Args, Parser, Subcommand};

use crate::{
    geometry::check_wkb,
    overture_types::{get_schema_json, schema_types},
};

mod geometry;
mod overture_types;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CheckWkb(CheckWkbArgs),
    Schema,
    Schemafy,
}

#[derive(Args)]
struct CheckWkbArgs {
    bytes: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::CheckWkb(args) => {
            let bytes_string = &args.bytes;
            let bytes_array = serde_json::from_str::<Vec<u8>>(&bytes_string).expect("bytes array");
            check_wkb(bytes_array.as_slice());
            println!("Check WKB end");
        }
        Commands::Schema => {
            println!("Schema start");
            get_schema_json();
            println!("Schema end");
        }
        Commands::Schemafy => {
            println!("Schemafy start");
            schema_types();
            println!("Schemafy end");
        }
    }
}
