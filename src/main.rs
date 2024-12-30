use clap::Parser;

#[derive(Debug)]
enum Time {
    Second,
    Minute,
    Hour,
}

#[derive(Debug)]
enum Length {
    Centimeter,
    Meter,
    Kilometer,
}

#[derive(Debug)]
enum Mass {
    Gram,
    Kilogram,
    Ton,
}

#[derive(Debug)]
enum Temperature {
    Celsius,
    Fahrenheit,
    Kelvin,
}

#[derive(Parser)]
struct Converter {
    val: f64,

    #[arg(short = 'u', long = "unit")]
    unit: String,

    #[arg(short = 'f', long = "from")]
    from: String,

    #[arg(short = 't', long = "to")]
    to: String,
}

trait Convertible {
    fn convert(&self, to: &str, val: f64) -> f64;
}

impl Convertible for Time {
    fn convert(&self, to: &str, val: f64) -> f64 {
        match (self, to) {
            (Time::Second, "minute") => val / 60.0,
            (Time::Second, "hour") => val / 3600.0,
            (Time::Minute, "second") => val * 60.0,
            (Time::Minute, "hour") => val / 60.0,
            (Time::Hour, "second") => val * 3600.0,
            (Time::Hour, "minute") => val * 60.0,
            _ => panic!("Unsupported time unit conversion"),
        }
    }
}

impl Convertible for Length {
    fn convert(&self, to: &str, val: f64) -> f64 {
        match (self, to) {
            (Length::Centimeter, "meter") => val / 100.0,
            (Length::Centimeter, "kilometer") => val / 100000.0,
            (Length::Meter, "centimeter") => val * 100.0,
            (Length::Meter, "kilometer") => val / 1000.0,
            (Length::Kilometer, "centimeter") => val * 100000.0,
            (Length::Kilometer, "meter") => val * 1000.0,
            _ => panic!("Unsupported length unit conversion"),
        }
    }
}

impl Convertible for Mass {
    fn convert(&self, to: &str, val: f64) -> f64 {
        match (self, to) {
            (Mass::Gram, "kilogram") => val / 1000.0,
            (Mass::Gram, "ton") => val / 1_000_000.0,
            (Mass::Kilogram, "gram") => val * 1000.0,
            (Mass::Kilogram, "ton") => val / 1000.0,
            (Mass::Ton, "gram") => val * 1_000_000.0,
            (Mass::Ton, "kilogram") => val * 1000.0,
            _ => panic!("Unsupported mass unit conversion"),
        }
    }
}

impl Convertible for Temperature {
    fn convert(&self, to: &str, val: f64) -> f64 {
        match (self, to) {
            (Temperature::Celsius, "fahrenheit") => val * 9.0 / 5.0 + 32.0,
            (Temperature::Celsius, "kelvin") => val + 273.15,
            (Temperature::Fahrenheit, "celsius") => (val - 32.0) * 5.0 / 9.0,
            (Temperature::Fahrenheit, "kelvin") => (val - 32.0) * 5.0 / 9.0 + 273.15,
            (Temperature::Kelvin, "celsius") => val - 273.15,
            (Temperature::Kelvin, "fahrenheit") => (val - 273.15) * 9.0 / 5.0 + 32.0,
            _ => panic!("Unsupported temperature unit conversion"),
        }
    }
}

fn parse_unit(unit: &str, from: &str) -> Box<dyn Convertible> {
    match unit {
        "time" => Box::new(match from {
            "second" => Time::Second,
            "minute" => Time::Minute,
            "hour" => Time::Hour,
            _ => panic!("Unknown time unit"),
        }),
        "length" => Box::new(match from {
            "centimeter" => Length::Centimeter,
            "meter" => Length::Meter,
            "kilometer" => Length::Kilometer,
            _ => panic!("Unknown length unit"),
        }),
        "mass" => Box::new(match from {
            "gram" => Mass::Gram,
            "kilogram" => Mass::Kilogram,
            "ton" => Mass::Ton,
            _ => panic!("Unknown mass unit"),
        }),
        "temperature" => Box::new(match from {
            "celsius" => Temperature::Celsius,
            "fahrenheit" => Temperature::Fahrenheit,
            "kelvin" => Temperature::Kelvin,
            _ => panic!("Unknown temperature unit"),
        }),
        _ => panic!("Unknown unit type"),
    }
}

fn main() {
    let args = Converter::parse();

    let unit = parse_unit(&args.unit, &args.from);
    let result = unit.convert(&args.to, args.val);
    println!("Converted value: {}", result);
}