#!/bin/sh

export DEBIAN_FRONTEND=noninteractive

apt update -y
apt upgrade -y

apt install -y curl gcc

curl https://sh.rustup.rs -sSf | sh -s -- -y

. $HOME/.cargo/env

cargo test
