# 2019-06-17 updates
install ubuntu 18.04 and WSL.

```
sudo apt install curl git make cargo
curl https://sh.rustup.rs -sSf | RUSTUP_INIT_SKIP_PATH_CHECK=yes sh
rustup update
# cargo +nightly with error then
rustup toolchain install nightly 
rustup default nightly
# to use nightly toolchain by default, at which point you can omit +nightly from all commands
```

#----

<img src="http://www.redaktion.tu-berlin.de/fileadmin/fg308/icons/projekte/logos/ZoKrates_logo.svg" width="100%" height="180">

# ZoKrates

[![Join the chat at https://gitter.im/ZoKrates/Lobby](https://badges.gitter.im/ZoKrates/Lobby.svg)](https://gitter.im/ZoKrates/Lobby?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![CircleCI master](https://img.shields.io/circleci/project/github/Zokrates/ZoKrates/master.svg?label=master)](https://circleci.com/gh/Zokrates/ZoKrates/tree/master)
[![CircleCI develop](https://img.shields.io/circleci/project/github/Zokrates/ZoKrates/develop.svg?label=develop)](https://circleci.com/gh/Zokrates/ZoKrates/tree/develop)

ZoKrates is a toolbox for zkSNARKs on Ethereum.

_This is a proof-of-concept implementation. It has not been tested for production._

## Getting Started

```bash
curl -LSfs get.zokrat.es | sh
```

Have a look at the [documentation](https://zokrates.github.io/) for more information about using ZoKrates.  
A getting started tutorial can be found [here](https://zokrates.github.io/sha256example.html).

## Getting Help

If you run into problems, ZoKrates has a Gitter room. You can come ask for help on [Gitter](https://gitter.im/ZoKrates/Lobby).

## License

ZoKrates is released under the GNU Lesser General Public License v3.

## Contributing

We happily welcome contributions. You can either pick an existing issue, or reach out on [Gitter](https://gitter.im/ZoKrates/Lobby).

Unless you explicitly state otherwise, any contribution you intentionally submit for inclusion in the work shall be licensed as above, without any additional terms or conditions.
