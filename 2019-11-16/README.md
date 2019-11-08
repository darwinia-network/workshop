# 使用Rust和Substrate开发简单的加密猫游戏

## Substrate环境部署
### 安装步骤
参考[Minimal substrate development environment setup](https://github.com/darwinia-network/workshop/blob/master/minimal-substrate-development-environment-setup/README.md)

## 创建Substrate节点
将工程名暂定为substratekitties
```sh
 #使用脚本文件创建待runtime的模板节点
 substrate-node-new substratekitties <your name>
 cd substratekitties
```
## 下载已完成的加密猫合约代码
```sh
git clone https://github.com/darwinia-network/workshop.git
#将下载仓库中的substratekitties.rs赋值复制到新节点的runtime/src下
cp <your path>/workshop/substratekitties.rs <your path>/substratekitties/runtime/src/
```
## 在lib.rs添加新模块
```rust
//添加新Module
mod substratekitties;
//实现trait
impl substratekitties::Trait for Runtime{
	type Event = Event;
}
//在construct_runtime!中声明我们的模块
construct_runtime!{
	puub enum Runtime with Log(InternalLog: DigestItem<Hash, AuthorityId, AuthoritySignature>)
	where ...
		...
		...{
		Substratekitties : substratekitties::{Module,Call,Storage,Event<T>};
	}
}
```

## 编译运行
```sh
#打开工程目录
cd substratekitties
#编译成Wasm文件
#如果是第一次使用节点，要先执行./scripts/init.sh
./scripts/build.sh
#编译成二进制文件
cargo build --release
#启动节点
./target/release/substratekitties --dev
#如果此次行为属于对链进行修改后的更新，可先清空
./target/release/substratekitties purge-chain --dev
```

## 查看
访问https://polkadot.js.org/apps/#/explorer

Settings -> remote node/endpoint to connect to -> Locol Host
![example](https://github.com/Eanam/workshop/blob/master/2019-11-16/pics/Setting.png?raw=true)

## 链的升级
区别于上一种对链先清空在更新的方法，Substrate提供了一种更简单的升级的方法，可以在链启动后进行无分叉的实时链升级
```sh
#编译成Wasm Runtime文件
./scripts/build.sh
#访问<your path>/substratekitties/runtime/wasm/target/wasm32-unknown-unknown/release/
#可观察到一个新的compact Wasm Binary
```
将该Wasm文件拖拽到如下界面指定位置后进行提交，刷新后则可以观察到更新的内容：
![update](https://raw.githubusercontent.com/Eanam/workshop/master/2019-11-16/pics/Update.png)

:heavy_exclamation_mark: 调用要拥有sudo的权限，可在Sudo -> Set sudo key下查看





