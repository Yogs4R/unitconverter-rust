use crate::riwayat;

pub fn convert_lenght(from: &str, to: &str, value: f64) {
    let from_lower = from.trim().to_lowercase();
    let to_lower = to.trim().to_lowercase();

    let result = match (from_lower.as_str(), to_lower.as_str()) {
        // Same unit conversion
        ("cm" | "centimeter", "cm" | "centimeter") => Some(value),
        ("km" | "kilometer", "km" | "kilometer") => Some(value),
        ("inch", "inch") => Some(value),
        ("ft" | "feet", "ft" | "feet") => Some(value),
        ("miles", "miles") => Some(value),

        // Centimeter to Kilometer
        ("cm" | "centimeter", "km" | "kilometer") => Some(value / 100000.0),
        // Centimeter to Inch
        ("cm" | "centimeter", "inch") => Some(value / 2.54),
        // Centimeter to Feet
        ("cm" | "centimeter", "ft" | "feet") => Some(value / 30.48),
        // Centimeter to Miles
        ("cm" | "centimeter", "miles") => Some(value / 160934.0),

        // Kilometer to Centimeter
        ("km" | "kilometer", "cm" | "centimeter") => Some(value * 100000.0),
        // Kilometer to Inch
        ("km" | "kilometer", "inch") => Some(value * 39370.1),
        // Kilometer to Feet
        ("km" | "kilometer", "ft" | "feet") => Some(value * 3280.84),
        // Kilometer to Miles
        ("km" | "kilometer", "miles") => Some(value / 1.60934),

        // Inch to Centimeter
        ("inch", "cm" | "centimeter") => Some(value * 2.54),
        // Inch to Kilometer
        ("inch", "km" | "kilometer") => Some(value / 39370.1),
        // Inch to Feet
        ("inch", "ft" | "feet") => Some(value / 12.0),
        // Inch to Miles
        ("inch", "miles") => Some(value / 63360.0),

        // Feet to centimeter
        ("ft" | "feet", "cm" | "centimeter") => Some(value * 30.48),
        // Feet to Kilometer
        ("ft" | "feet", "km" | "kilometer") => Some(value / 3280.84),
        // Feet to Inch
        ("ft" | "feet", "inch") => Some(value * 12.0),
        // Feet to Miles
        ("ft" | "feet", "miles") => Some(value / 5280.0),

        // Miles to Centimeter
        ("miles", "cm" | "centimeter") => Some(value * 160934.0),
        // Miles to Kilometer
        ("miles", "km" | "kilometer") => Some(value * 1.60934),
        // Miles to Inch
        ("miles", "inch") => Some(value * 63360.0),
        // Miles to Feet
        ("miles", "ft" | "feet") => Some(value * 5280.0),

        // Wrong units
        _ => {
            println!("Error: Satuan panjang '{}' atau '{}' tidak dikenali.", from, to);
            return;
        }
    };

    println!("{:.1} {} = {:.1} {}", value, from.to_uppercase(), result.unwrap(), to.to_uppercase());

    riwayat::save_riwayat(from, to, value, result.unwrap());
}