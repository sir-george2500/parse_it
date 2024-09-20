# JSON Parser in Rust - `parse_it`
![Parse_it](https://github.com/sir-george2500/custome_images/blob/main/images/parse_it.png)

Welcome to **parse_it**, a simple JSON parser that can handle string keys and values, as well as nested objects and arrays. This parser reads JSON data from a file or standard input, validating its structure with the precision of a Swiss watchmaker. 

It's simple, straightforward, and efficient—of course, because it’s written in Rust! (by the way)

## Features

- Parse JSON objects containing:
  - String keys and values
  - Numeric values
  - Boolean values (`true`, `false`)
  - `null`
  - Nested objects
  - Arrays

## Getting Started

### Prerequisites

- Rust installed on your machine. If you haven’t installed Rust yet, follow the [official installation guide](https://www.rust-lang.org/tools/install). It’s easier than explaining recursion to your cat!

### Cloning the Repository

Clone this repository to your local machine:

```bash
git clone https://github.com/sirgeorge2500/parse_it.git
cd parse_it
```

## Using the tool

1. **Create a Test File**: Create a file named `test.json` and include your JSON content. For example:

    ```json
    {
        "key1": true,
        "key2": false,
        "key3": null,
        "key4": "value",
        "key5": 101,
        "key6": {},
        "key7": []
    }
    ```

### then run this command 
```bash 

cargo run -q test_input.json

```

## Tool Features

- **Input Handling**: Accepts both file paths and inline JSON strings.
- **Error Reporting**: Provides clear error messages for invalid JSON structures.
- **Extensible**: Can be extended to include more features, such as additional data types or validation checks.

```
```
