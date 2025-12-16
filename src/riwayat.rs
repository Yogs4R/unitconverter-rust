use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct History {
    pub from: String,
    pub to: String,
    pub value: f64,
    pub result: f64,
}

pub fn load_riwayat() -> Vec<History> {
    match fs::read_to_string("riwayat.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

pub fn save_riwayat(from: &str, to: &str, value: f64, result: f64) {
    let mut riwayat = load_riwayat();

    riwayat.push(History {
        from: from.to_string(),
        to: to.to_string(),
        value,
        result,
    });

    let json = serde_json::to_string_pretty(&riwayat).expect("Gagal mengonversi riwayat ke JSON");
    fs::write("riwayat.json", json).expect("Gagal menyimpan riwayat");
}

pub fn print_riwayat() {
    let riwayat = load_riwayat();

    if riwayat.is_empty() {
        println!("Tidak ada riwayat konversi.");
        return;
    }

    println!("=== Riwayat Konversi ===");
    for (i, item) in riwayat.iter().enumerate() {
        let symbol_from = match item.from.as_str() {
            "c" | "celsius" | "f" | "fahrenheit" | "k" | "kelvin" => "°",
            _ => "",
        };

        let symbol_to = match item.to.as_str() {
            "c" | "celsius" | "f" | "fahrenheit" | "k" | "kelvin" => "°",
            _ => "",
        };
        
        println!("{}. {:.1} {}{} -> {:.1} {}{}", i + 1, item.value, symbol_from, item.from.to_uppercase(), item.result, symbol_to, item.to.to_uppercase());
    }
    println!("========================");
}

pub fn clear_riwayat() {
    let empty_riwayat: Vec<History> = Vec::new();
    let json = serde_json::to_string_pretty(&empty_riwayat).expect("Gagal mengonversi riwayat ke JSON");
    fs::write("riwayat.json", json).expect("Gagal menghapus riwayat");
    println!("Riwayat konversi telah dihapus.");
}