use alloy_primitives::{I256, U256};
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use colored::*;
use std::{ops::Add, str};

/// Convert a uint256 value to wei, gwei and ether
#[derive(Debug)]
pub struct HexConverter {
    pub value: String,
}

impl HexConverter {
    pub fn new(_value: String, from_hex: bool) -> Self {
        if from_hex {
            let _value = if _value.starts_with("0x") {
                _value[2..].to_string()
            } else {
                _value
            };
    
            // Add a 0 if len is not odd
            let _value = if _value.len() % 2 != 0 {
                format!("0{}", _value)
            } else {
                _value
            };
            return Self { value: _value };
        }

        Self { value: _value }
    }

    pub fn print_from_hex_conversion(&self) {
        let bytes = self.hex_to_bytes(self.value.as_str()).unwrap();

        // Bytes to string
        let string_convertion = str::from_utf8(&bytes.clone())
            .unwrap_or("Invalid UTF-8".red().to_string().as_str())
            .to_string();

        // Bytes to uint256
        let uint_convertion: String = if bytes.len() <= 32 {
            U256::from_be_slice(bytes.clone().as_slice()).to_string()
        }
        else{
            "Value too large".red().to_string()
        };
        
        let int_convertion: String = if bytes.len() <= 32 {
            I256::try_from_be_slice(bytes.clone().as_slice()).unwrap().to_string()
        }
        else{
            "Value too large".red().to_string()
        };

        // Bytes to float
        let float_convertion: String = if bytes.len() == 8 {
            self.format_float(f64::from_be_bytes(bytes.as_slice().try_into().unwrap())).to_string()
        }
        else {
            "Invalid float".red().to_string()
        };

        let table = vec![
            vec![
                string_convertion.cell().justify(Justify::Right),
                uint_convertion.clone().cell().justify(Justify::Right),
                int_convertion.clone().cell().justify(Justify::Right),
                float_convertion.cell().justify(Justify::Right),
            ],
            vec![
                "".cell().justify(Justify::Right),
                self.format_number(if bytes.len() <= 32 {uint_convertion} else {String::from("")})
                    .cell()
                    .justify(Justify::Right),
                self.format_number(if bytes.len() <= 32 {int_convertion} else {String::from("")})
                    .cell()
                    .justify(Justify::Right),
                "".cell().justify(Justify::Right),
            ],
        ]
        .table()
        .title(vec![
            "String".blue().cell().bold(true),
            "Unsigned int".blue().cell().bold(true),
            "int256".blue().cell().bold(true),
            "Float".blue().cell().bold(true),
        ])
        .bold(true);

        println!("Hex conversion for: {}", format!("0x{}", self.value).green());
        assert!(print_stdout(table).is_ok());
    }

    pub fn print_to_hex_conversion(&self) {
        let is_float = self.value.parse::<f64>().is_ok();
        let is_uint = U256::from_str_radix(self.value.as_str(), 10).is_ok();
        let is_int = I256::from_dec_str(self.value.as_str()).is_ok();

        let mut float_convertion: String = format!("{}", "Invalid float".red());
        let mut uint_convertion: String = format!("{}", "Invalid uint".red());
        let mut int_convertion: String = format!("{}", "Invalid int".red());

        if is_float {
            let float = self.value.parse::<f64>().unwrap();
            float_convertion = self.bytes_to_hex(float.to_be_bytes().to_vec());
        }
        if is_uint {
            let uint = U256::from_str_radix(self.value.as_str(), 10).unwrap();
            uint_convertion = self.bytes_to_hex(uint.to_be_bytes_vec());
        }
        if is_int {
            let int = I256::from_dec_str(self.value.as_str()).unwrap();
            int_convertion = self.bytes_to_hex(int.to_be_bytes::<32>().to_vec());
        }

        let string_convertion = self.bytes_to_hex(self.value.as_bytes().to_vec());

        let table = vec![
            vec![
                string_convertion.cell().justify(Justify::Right),
                uint_convertion.cell().justify(Justify::Right),
                int_convertion.cell().justify(Justify::Right),
                float_convertion.cell().justify(Justify::Right),
            ]
        ]
        .table()
        .title(vec![
            "String".blue().cell().bold(true),
            "Uint 256".blue().cell().bold(true),
            "Int 256".blue().cell().bold(true),
            "Float".blue().cell().bold(true)
        ])
        .bold(true);

        assert!(print_stdout(table).is_ok());
    }

    fn hex_to_bytes(&self, s: &str) -> Option<Vec<u8>> {
        // Check if s is a valid hex string
        if s.chars().all(|c| c.is_digit(16)) {
            (0..s.len())
                .step_by(2)
                .map(|i| {
                    s.get(i..i + 2)
                        .and_then(|sub| u8::from_str_radix(sub, 16).ok())
                })
                .collect()
        }
        else{
            println!("{}", "ERR: Invalid hex string".red());
            std::process::exit(1);
        }
    }

    fn bytes_to_hex(&self, bytes: Vec<u8>) -> String {
        let mut result = String::from("");

        let mut started = false;
        for byte in bytes {
            if !started && byte == 0 { continue; }
            if !started && byte != 0 { started = true; }
            let hex = format!("{:02x}", byte);
            result.push_str(hex.as_str());
        }

        format!("0x{}", result)
    }

    fn format_float(&self, float: f64) -> String {
        format!("{:.2}", float)
    }

    fn format_number(&self, number: String) -> String {
        if number.len() == 0 {
            " ".to_string()
        }
        else {
            number
                .as_bytes()
                .rchunks(3)
                .rev()
                .map(std::str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .unwrap()
                .join(",")
        }
    }
}
