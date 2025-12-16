mod suhu_converter;
mod riwayat;
mod panjang_converter;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Unit Converter", version = "1.0", about = "Units conversion tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert units
    Convert {
        #[arg(long)]
        from: String,

        #[arg(long)]
        to: String,

        #[arg(long)]
        value: f64,
    },

    /// Show conversion history
    History,

    /// List supported units
    List,

    /// Delete conversion history
    Clear,
}

fn get_unit_type(unit: &str) -> &str {
    match unit.trim().to_lowercase().as_str() {
        "c" | "celsius" | "f" | "fahrenheit" | "k" | "kelvin" => "suhu",
        "cm" | "centimeter" | "km" | "kilometer" | "inch" | "ft" | "feet" | "miles" => "panjang",
        _ => "unknown",
    }
}


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Convert { from, to, value } => {
            let from_type = get_unit_type(&from);
            let to_type = get_unit_type(&to);

            if to_type == "unknown" || from_type == "unknown" {
                println!("Error: Satuan '{}' atau '{}' tidak dikenali.", from, to);
                return;
            }

            if from_type != to_type {
                println!("Error: Tidak dapat mengonversi antara jenis satuan yang berbeda ('{}' dan '{}').", from, to);
                return;
            }

            match from_type {
                "suhu" => suhu_converter::convert_temperature(&from, &to, value),
                "panjang" => panjang_converter::convert_lenght(&from, &to, value),
                _ => println!("Error: Jenis satuan '{}' tidak dikenali.", from_type),
            }
        }
        Commands::History => {
            riwayat::print_riwayat(); 
        }

        Commands::List => {
            println!("Satuan Suhu yang didukung:");
            
            let units = [
                ("suhu", "celsius"),
                ("suhu", "fahrenheit"),
                ("suhu", "kelvin"),
                ("panjang", "centimeter"),
                ("panjang", "kilometer"),
                ("panjang", "inch"),
                ("panjang", "feet"),
                ("panjang", "miles"),
            ];

            for (i, (unit_type, unit_name)) in units.iter().enumerate() {
                println!("{}. {} - {}", i + 1, unit_type, unit_name);
            }
        }
        Commands::Clear => {
            riwayat::clear_riwayat();
        }
    }
}
