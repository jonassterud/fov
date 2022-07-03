![FOV](media/logo/cover.png)

## About
*FOV* is an application to help you get an overview of your finances, by combining data from different platforms. Currently, the main goal is to implement support for [SpareBank 1](https://www.sparebank1.no), [Nordnet](https://www.nordnet.no/no) and [Coinbase Pro](https://pro.coinbase.com/) using their APIs.

## Installation
You can download the application from the [releases](https://github.com/jonassterud/fov/releases) section here on GitHub.

## Configuration
For the application to get access to your financial data, you need to provide the different API keys in a file called `secrets.toml`. These tokens will allow this program to get a lot of control over the different platforms. Here is how the file is supposed to be structured:
```toml
sparebank1_access_token = "<YOUR SPAERBANK 1 API ACCESS TOKEN>"
cbp_key = <"YOUR COINBASE PRO API KEY">
cbp_secret = <"YOUR COINBASE PRO API SECRET">
cbp_passphrase = <"YOUR COINBASE PRO API PASSPHRASE">
```

## Contributing
Feel free to contribute!

Use *[Rustfmt](https://github.com/rust-lang/rustfmt)* and *[Clippy](https://github.com/rust-lang/rust-clippy)* before submitting a pull request.

Please use [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/).

## License
This project is licensed under the MIT License - see the [LICENSE](./LICENSE) for details.