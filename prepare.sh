#!/bin/sh

export DEBIAN_FRONTEND=noninteractive

apt update
apt upgrade

apt install -y curl gcc

curl https://sh.rustup.rs -sSf | sh -s -- -y

. $HOME/.cargo/env

cargo test
