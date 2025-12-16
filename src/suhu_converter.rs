use crate::riwayat;

pub fn convert_temperature(from: &str, to: &str, value: f64) {
    let from_lower = from.trim().to_lowercase();
    let to_lower = to.trim().to_lowercase();

    let result = match (from_lower.as_str(), to_lower.as_str()) {
        // Same unit conversion
        ("c" | "celsius", "c" | "celsius") => Some(value),
        ("f" | "fahrenheit", "f" | "fahrenheit") => Some(value),
        ("k" | "kelvin", "k" | "kelvin") => Some(value),

        // Celsius to Fahrenheit
        ("c" | "celsius", "f" | "fahrenheit") => Some((value * 9.0 / 5.0) + 32.0),
        // Celsius to Kelvin
        ("c" | "celsius", "k" | "kelvin") => Some(value + 273.15),

        // Fahrenheit to Celsius
        ("f" | "fahrenheit", "c" | "celsius") => Some((value - 32.0) * 5.0 / 9.0),
        // Fahrenheit to Kelvin
        ("f" | "fahrenheit", "k" | "kelvin") => Some((value - 32.0) * 5.0 / 9.0 + 273.15),

        // Kelvin to Celsius
        ("k" | "kelvin", "c" | "celsius") => Some(value - 273.15),
        // Kelvin to Fahrenheit
        ("k" | "kelvin", "f" | "fahrenheit") => Some((value - 273.15) * 9.0 / 5.0 + 32.0),

        // Wrong units
        _ => {
            println!("Error: Satuan suhu '{}' atau '{}' tidak dikenali.", from, to);
            return;
        }
    };

    println!("{:.1} °{} = {:.1} °{}", value, from.to_uppercase(), result.unwrap(), to.to_uppercase());

    riwayat::save_riwayat(from, to, value, result.unwrap());
}