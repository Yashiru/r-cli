use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use colored::*;

/// Convert a uint256 value to wei, gwei and ether
#[derive(Debug)]
pub struct WadConverter {
    pub value: f64,
}

impl WadConverter {
    pub fn new(_value: f64) -> Self {
        Self { value: _value }
    }

    pub fn print_conversion(&self) {
        let to_wei = self.value * 1e18;
        let to_gwei = self.value * 1e9;
        let from_gwei = self.value / 1e9;
        let from_wei = self.value / 1e18;

        let table = vec![
            vec![
                self.value
                    .to_string()
                    .green()
                    .cell()
                    .bold(true)
                    .justify(Justify::Right),
                from_gwei.cell().justify(Justify::Right),
                from_wei.cell().justify(Justify::Right),
            ],
            vec![
                to_gwei.cell().justify(Justify::Right),
                self.value
                    .to_string()
                    .green()
                    .cell()
                    .bold(true)
                    .justify(Justify::Right),
                from_gwei.cell().justify(Justify::Right),
            ],
            vec![
                to_wei.cell().justify(Justify::Right),
                to_gwei.cell().justify(Justify::Right),
                self.value
                    .to_string()
                    .green()
                    .cell()
                    .bold(true)
                    .justify(Justify::Right),
            ],
        ]
        .table()
        .title(vec![
            "From ether".blue().cell().bold(true),
            "From gwei".blue().cell().bold(true),
            "From wei".blue().cell().bold(true),
        ])
        .bold(true);

        assert!(print_stdout(table).is_ok());
    }
}
