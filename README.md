# JSON Parser and Search Utility

[![forthebadge](./assets/svg/made-with-rust.svg)](https://www.rust-lang.org/)
[![forthebadge](./assets/svg/use-asciinema.svg)](https://asciinema.org/)
[![forthebadge](./assets/svg/use-forthebadge.svg)](https://forthebadge.com)

## Description

This is a Rust-based command-line utility designed to parse, analyze, and search JSON files efficiently. It provides a simple way to explore JSON structures, locate specific values, and handle complex data formats. The program supports flexible input methods (file paths or stdin) and highlights search results for better readability.

## Features

- Parse and pretty-print JSON files.
- Search for specific values within JSON structures.
- Display the full path (keys or indices) to the matched values.
- Show indices when matches occur within arrays.
- Support for input via file paths or standard input (stdin).
- Color-coded output for search results.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

### Clone and Build

1. Clone this repository:
   ```bash
   git clone https://github.com/nightwolf-1/json_parser.git
   cd json-parser
    ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. The binary will be avialable in the target/release directory:
    ```bash
    ./target/release/json_parser
    ```

# Usage: 

## To print all json on terminal
![Print all json](./assets/gifs/print_all_json.gif)

#### You can also use standard input

![Print all json stdin](./assets/gifs/print_all_json_stdin.gif)

### To print a specific key of json
![Exemple key](./assets/gifs/json_key_exemple.gif)

#### You can also use standard input

![Exemple key stdin](./assets/gifs/json_key_exemple_stdin.gif)

### To print a specific index of key in json
![Exemple specific index of key](./assets/gifs/json_key_index_exemple.gif)

#### You can also use standard input

![Exemple specific index of key stdin](./assets/gifs/json_key_index_exemple_stdin.gif)

## To print a nested oject
![Exemple nested object](./assets/gifs/json_nested_object.gif)

#### You can also use standard input

![Exemple nested Object stdin](./assets/gifs/json_nested_object_stdin.gif)

## To print a nested object with specific tab index
![Exemple nested object index](./assets/gifs/json_nested_object_index.gif)

#### You can also use standard input

![Exemple nested Object stdin](./assets/gifs/json_nested_object_index_stdin.gif)

## To print all occurences of a value
![Exemple print all occurences of a value](./assets/gifs/json_all_occurences_value.gif)

#### You can also use standard input

![Exemple print all occurences of a value stdin](./assets/gifs/json_all_occurences_value_stdin.gif)
