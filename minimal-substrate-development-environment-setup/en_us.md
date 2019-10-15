## Setup substrate (*nix)

### Script (not recommend)

```sh
curl https://getsubstrate.io -sSf | bash -s -- --fast
```

### Manually (recommend)

```sh
# due to the hex-literal compiling error we use the `nighyly-2019-07-14` version
# if rust not in your system
curl https://sh.rustup.rs -sSf | sh
> 2
>
> nightly-2019-07-14
>
>
source $HOME/.cargo/env
# if rust is already installed
rustup default nightly-2019-07-14

# wasm
rustup target add wasm32-unknown-unknown
cargo install --git https://github.com/alexcrichton/wasm-gc

# build dependency
# brew user
brew intsall cmake git llvm openssl pkg-config 
# nix-pkg user
nix-env -i cmake git llvm openssl pkg-config
# archlinux user
pacman -S clang cmake gcc git openssl pkgconf

# substrate-up
git clone https://github.com/paritytech/substrate-up
cp -a substrate-up/substrate-* ~/.cargo/bin
cp -a substrate-up/polkadot-* ~/.cargo/bin
source $HOME/.cargo/env

# custom web ui dependency (optional)
# brew user
brew install nodejs yarn
# nix-pkg user
nix-env -i nodejs yarn
```

## Setup substrate (Windows)

[https://github.com/paritytech/substrate#612-windows](https://github.com/paritytech/substrate#612-windows)

## Setup project

```sh
# ready-to-hack substrate node with a template runtime module
substrate-node-new erc20-demo <your name>
cd erc20-demo
./target/release/erc20-demo --dev

# custom web ui (optional)
substrate-ui-new erc20-demo
cd erc20-demo-ui
yarn run dev
```
