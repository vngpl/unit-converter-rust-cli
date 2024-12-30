# Unit Converter

This project is a simple unit converter written in Rust. It supports conversions for time, length, mass, and temperature units.

## Supported Units

### Time
* second
* minute
* hour

### Length
* centimeter
* meter
* kilometer

### Mass
* gram
* kilogram
* ton

### Temperature
* celsius
* fahrenheit
* kelvin

## Usage

To use the converter, clone the repository and navigate to the project directory, then run the following command:

```bash
cargo run -- <value> --unit <unit> --from <from_unit> --to <to_unit>
```

Alternatively, using the short-hand notation:

```bash
cargo r -- <value> -u <unit> -f <from_unit> -t <to_unit>
```