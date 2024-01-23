use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;

use commands::wad::wad_converter::WadConverter;
use commands::hex::hex_converter::HexConverter;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert a value to wei, gwei and ether
    Wad {
        #[arg(allow_hyphen_values = true)]
        value: f64,
    },
    /// Alias for wad
    Wei {
        #[arg(allow_hyphen_values = true)]
        value: f64,
    },
    /// Alias for wad
    Gwei {
        #[arg(allow_hyphen_values = true)]
        value: f64,
    },
    /// Convert a value from hex to string, uint256, int256 and float
    FromHex {
        #[arg(allow_hyphen_values = true)]
        value: String,
    },
    /// Alias for from_hex
    Fhex {
        #[arg(allow_hyphen_values = true)]
        value: String,
    },
    /// Convert a value to hex
    ToHex {
        #[arg(allow_hyphen_values = true)]
        value: String,
    },
    /// Alias for to_hex
    Thex {
        #[arg(allow_hyphen_values = true)]
        value: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Wad { value } => {
            WadConverter::new(*value).print_conversion();
        }
        Commands::Wei { value } => {
            WadConverter::new(*value).print_conversion();
        }
        Commands::Gwei { value } => {
            WadConverter::new(*value).print_conversion();
        }
        Commands::FromHex { value } => {
            HexConverter::new(value.to_string(), true).print_from_hex_conversion();
        }
        Commands::Fhex { value } => {
            HexConverter::new(value.to_string(), true).print_from_hex_conversion();
        }
        Commands::ToHex { value } => {
            HexConverter::new(value.to_string(), false).print_to_hex_conversion();
        }
        Commands::Thex { value } => {
            HexConverter::new(value.to_string(), false).print_to_hex_conversion();
        }
    }
    Ok(())
}
