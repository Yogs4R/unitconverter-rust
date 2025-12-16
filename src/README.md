# 🔄 UnitConv - CLI Unit Converter

A smart command-line tool to convert Temperature and Length units, built with Rust.

![Rust](https://img.shields.io/badge/Made%20with-Rust-orange?style=flat-square&logo=rust)

## ✨ Features
* **Smart Conversion**: Automatically detects if input is Temperature or Length.
* **Temperature**: Celsius, Fahrenheit, Kelvin (with `°` symbol support).
* **Length**: Meter, Kilometer, Centimeter, Millimeter, Miles, Inch, Feet.
* **History**: Saves your conversion history automatically.
* **Management**: Commands to view list of units and clear history.

## 🚀 Usage

### 1. Convert Units
```bash
# Temperature
cargo run -- convert --from c --to f --value 100

# Length
cargo run -- convert --from km --to miles --value 50

