# Probachain

A chain that is governed by AI !!!


## Installation

```bash
wget https://download.pytorch.org/libtorch/cu121/libtorch-cxx11-abi-shared-with-deps-2.2.0%2Bcu121.zip
unzip libtorch-cxx11-abi-shared-with-deps-2.2.0+cu121.zip

export LIBTORCH=$(pwd)/libtorch
export LD_LIBRARY_PATH=${LIBTORCH}/lib
```

```bash
cargo build --release
```

## Run

```bash
./target/release/node-template --dev --rpc-cors=all --unsafe-rpc-external --alice
```