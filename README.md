# oohid

## Description
oohid is a fast and efficient command-line interface (CLI) tool for generating Universally Unique Identifiers (UUIDs) in various formats. It allows users to output UUIDs directly to a file or standard output (stdout). Features options for custom formatting, duplicate checking, and verbose output.

## Features
- Generate multiple UUIDs in a single command.
- Custom formatting options for UUIDs.
- Output to a file or stdout.
- Check for and remove duplicate UUIDs.
- Verbose mode for additional output information.

## Characteristics
- Pretty fast, as it is dependent on the work of wiser people. Uses Rayon for embarassingly parallel tasks, probably some more optimization work to do. SIMD or assembly-level stuff or cache optimization. ~1 mil/s on my MacBook Pro w. M2 Pro.
- Formatting options suitable for Python/Rust.
- For the extra-paranoid, duplicate checking.

## Installation
To install oohid, ensure you have Rust and Cargo installed on your system. Then run the following command:

```
cargo install oohid
```

## Usage

### Basic Usage
To generate a single UUID and print it to stdout:

```
oohid
```

### Generating Multiple UUIDs
Generate 5 UUIDs:

```
oohid --count 5
```
or
```
oohid -c 5
```
### Specifying Format
Specify a custom format for the UUIDs (`ul` for bare with comma, `q` for quoted, etc.):

```
oohid --count 5 --format ul
```
or
```
oohid -c 5 -f ul
```
### Output to a File
Direct the output to a file:

```
oohid --count 10 --output uuids.txt
```
or
```
oohid -c 10 -o uuids.txt
```

### Checking for Duplicates
Check and remove duplicate UUIDs:

```
oohid -c 100 --check
```

### Verbose Output
Display benchmarking and checking results:

```
oohid -c 100000000 -f qlbl -o uuids.txt --check --verbose
```

## Configuration
Customize the behavior of oohid through command-line flags. Refer to the help command for more details:

```
oohid --help
```

## Contributing
Forks and contributions to oohid are welcome.

## License
oohid is distributed under the Unlicense. Do whatever you wish, but attributions would be nice; I am a newb.

## Author
[Younghyun Chi](https://www.linkedin.com/in/younghyun-chi-a60b59a9/)