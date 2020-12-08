#!/bin/sh

export DEBIAN_FRONTEND=noninteractive

apt update
apt upgrade

apt install -y rustc

cargo test
