# Case Converter

* convert snake case to camel case
* convert camel case to snake case

## Installation

```
cargo add case_converter
```

## Example for Convert snake case to camel case
```
use case_converter;

fn main() {
    let result = snake_to_camel("snake_to_camel");
    println!("{}", result); // → snakeToCamel
}
```

## Example for Convert camel case to snake case
```
use case_converter;

fn main() {
    let result = camel_to_snake("camelToSnake");
    println!("{}", result); // → camel_to_snake
}
```