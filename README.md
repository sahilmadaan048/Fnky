# Fnky

![Fnky Logo](Logo/logo.png)

Fnky (pronounced "funky") is a lightweight, expressive interpreter built in Rust that emphasizes functional execution. With `fn` at its heart, Fnky ensures your code runs smoothly, safely, and with syntactic style. Whether you're composing scripts or working with expressions, Fnky adds flair to your coding experience.

## Features

- **Functional Core**: Designed with function-based execution at its center.
- **Lightweight**: Minimalistic design for quick and efficient interpretation.
- **Expressive Syntax**: Write clear and concise code with Fnky's stylish syntax.
- **Safety**: Built with Rust's safety guarantees to ensure reliable code execution.

## Installation

To install Fnky, ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system. Then, run:

```bash
git clone https://github.com/sahilmadaan048/Fnky.git
cd Fnky
cargo build --release
```

The compiled binary will be located in the `target/release` directory.

## Usage

To start using Fnky, execute the compiled binary:

```bash
./target/release/fnky
```

You can then enter your Fnky code directly into the interpreter. Here's a simple example:

```fnky
fn add(a, b) {
    a + b
}

print(add(5, 3));
```

This will output:

```
8
```

## Contributing

We welcome contributions to Fnky! If you'd like to contribute, please fork the repository and submit a pull request with your changes.

## License

This project is licensed under the MIT License.

## Acknowledgments

Special thanks to the Rust community for their support and contributions to the ecosystem.

