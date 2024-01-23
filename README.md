# R CLI
## Introduction
This Rust CLI tool is designed for Ethereum Virtual Machine (EVM) development environments. It offers a suite of commands for easy conversion and manipulation of values commonly used in blockchain development, such as converting values to and from wei, gwei, ether, and hexadecimal formats.

## Features
**Value Conversion**: Convert values between wei, gwei, and ether with ease.
**Hexadecimal Conversion**: Easily convert values from hexadecimal to various data types and vice versa.
**Alias Commands**: Quick aliases for commonly used commands.

## Installation
To install this tool, you must clone this repo and run the `install` bash script. This will install the tool to your `/usr/local/bin` directory.
```bash
bash install
```

## Uninstallation
To uninstall this tool, you must remove the tool from your `/usr/local/bin` directory.
```bash
sudo rm /usr/local/bin/rr
```

## Usage
To use this tool, run the following command in your terminal:

```bash
Copy code
rr <COMMAND>
```

### Commands
- **`wad`/`wei`/`gwei`**: Convert a value to wei, gwei, and ether.
- **`from-hex`/`fhex`**: Convert a value from hex to string, uint256, int256, and float.
- **`to-hex`/`thex`**: Convert a value to hex.
-- **`help`**: Print help message or the help of the given subcommand(s).

### Options
`-h`, `--help`: Print help.

### Examples

#### Convert 1 ether to wei
```bash
rr wad 1

+---------------------+-------------+----------------------+
| From ether          | From gwei   | From wei             |
+---------------------+-------------+----------------------+
|                   1 | 0.000000001 | 0.000000000000000001 |
+---------------------+-------------+----------------------+
|          1000000000 |           1 |          0.000000001 |
+---------------------+-------------+----------------------+
| 1000000000000000000 |  1000000000 |                    1 |
+---------------------+-------------+----------------------+
```

#### Convert to hex
```bash
rr thex 1000

+------------+----------+---------+--------------------+
| String     | Uint 256 | Int 256 | Float              |
+------------+----------+---------+--------------------+
| 0x31303030 |   0x03e8 |  0x03e8 | 0x408f400000000000 |
+------------+----------+---------+--------------------+
```

#### Convert from hex
```bash
rr fhex 0x31303030

+--------+--------------+-------------+---------------+
| String | Unsigned int | int256      | Float         |
+--------+--------------+-------------+---------------+
|   1000 |    825241648 |   825241648 | Invalid float |
+--------+--------------+-------------+---------------+
|        |  825,241,648 | 825,241,648 |               |
+--------+--------------+-------------+---------------+
```

## License
MIT License
