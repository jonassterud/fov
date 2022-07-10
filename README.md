![FOV](media/logo/cover.png)

## About
*FOV* is an application to help you get an overview of your finances, by combining data from different platforms. Currently, the main goal is to implement support for [SpareBank 1](https://www.sparebank1.no), [Nordnet](https://www.nordnet.no/no), [Coinbase Pro](https://pro.coinbase.com/) and cryptocurrencies (with xPub keys).

## Installation
You can download the application from the [releases](https://github.com/jonassterud/fov/releases) section here on GitHub.

## Configuration
The file `src/config.toml` is used for configuration. When you run the application, it will prompt you for a password. This password is used to encrypt the configuration/decrypt existing configurations. Afterwards, the application will load `src/config.toml`. If an existing configuration exists (`src/secret.toml`), it will compare and combine them. Then, `src/config.toml` is removed of any sensitive data, and the final configuration is encrypted and added to `src/secret.toml`.

## Security
I can't vouch for the security of this application, and recommend that if you were to use it at this point, run it on a encrypted computer and on a safe network.

Here is what `src/config.toml` looks like:
```toml
# SpareBank 1 OAuth access token
sb1_access_token = ''
# Coinbase Pro API key
cbp_key = ''
# Coinbase Pro API secret
cbp_secret = ''
# Coinbase Pro API passphrase
cbp_passphrase = ''
# NOWNodes API key
nwn_key = ''
# Bitcoin xPub key
btc_xpub = ''
# Litecoin xPub key
ltc_xpub = ''
```

## Contributing
Feel free to contribute!

Use *[Rustfmt](https://github.com/rust-lang/rustfmt)* and *[Clippy](https://github.com/rust-lang/rust-clippy)* before submitting a pull request.

Please use [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/).

## License
This project is licensed under the MIT License - see the [LICENSE](./LICENSE) for details.