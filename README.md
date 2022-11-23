# Case Converter

* Convert snake case to camel case
* Convert kebab case to camel case
* Convert camel case to snake case
* Convert camel case to kebab case
* Convert kebab case to snake case
* Convert snake case to kebab case

## Installation

```
cargo add case_converter
```

## Example for Convert snake case to camel case
```
use case_converter::snake_to_camel;

fn main() {
    let result = snake_to_camel("snake_to_camel");
    println!("{}", result); // → snakeToCamel
}
```

## Example for Convert kebab case to camel case
```
use case_converter::kebab_to_camel;

fn main() {
    let result = kebab_to_camel("kebab-to-camel");
    println!("{}", result); // → kebabToCamel
}
```

## Example for Convert camel case to snake case
```
use case_converter::camel_to_snake;

fn main() {
    let result = camel_to_snake("camelToSnake");
    println!("{}", result); // → camel_to_snake
}
```

## Example for Convert camel case to kebab case
```
use case_converter::camel_to_kebab;

fn main() {
    let result = camel_to_kebab("camelToKebab");
    println!("{}", result); // → camel-to-kebab
}
```

## Example for Convert kebab case to snake case
```
use case_converter::kebab_to_snake;

fn main() {
    let result = kebab_to_snake("kebab-to-snake");
    println!("{}", result); // → kebab_to_snake
}
```

## Example for Convert snake case to kebab case
```
use case_converter::snake_to_kebab;

fn main() {
    let result = snake_to_kebab("snake_to_kebab");
    println!("{}", result); // → snake-to-kebab
}
```