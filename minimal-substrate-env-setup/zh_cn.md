## 部署 substrate (*nix 系统)

### 一键脚本 (不推荐)

```sh
curl https://getsubstrate.io -sSf | bash -s -- --fast
```

### 手动 (推荐)

```sh
# 由于某些版本编译依赖 hex-literal 会出问题，选择 `nighyly-2019-07-14` 版本
# 如果系统中没有 rust
curl https://sh.rustup.rs -sSf | sh
> 2
> PRESS Enter
> nightly-2019-07-14
> PRESS Enter
> PRESS Enter
source $HOME/.cargo/env
# 如果已经安装 rust
rustup default nightly-2019-07-14

# wasm 环境
rustup target add wasm32-unknown-unknown
cargo install --git https://github.com/alexcrichton/wasm-gc

# 构建环境
# brew 包管理器用户
brew intsall cmake git llvm openssl pkg-config
# nix-pkg 包管理器用户
nix-env -i cmake git llvm openssl pkg-config
# archlinux 系统用户
pacman -S clang cmake gcc git openssl pkgconf

# substrate-up 脚本，用于快速构建项目
git clone https://github.com/paritytech/substrate-up
cp -a substrate-up/substrate-* ~/.cargo/bin
cp -a substrate-up/polkadot-* ~/.cargo/bin
source $HOME/.cargo/env

# 自定义 web ui 依赖 (可选)
# brew 包管理器用户
brew install nodejs yarn
# nix-pkg 包管理器用户
nix-env -i nodejs yarn
```

## 部署 substrate (Windows 系统)

[https://github.com/paritytech/substrate#612-windows](https://github.com/paritytech/substrate#612-windows)

## 部署项目

```sh
# 创建一个带有 runtime 模块示例模版的 substrate 节点
substrate-node-new erc20-demo <your name>
cd erc20-demo
./target/release/erc20-demo --dev

# 创建自定义 web ui (可选)
substrate-ui-new erc20-demo
cd erc20-demo-ui
yarn run dev
```
