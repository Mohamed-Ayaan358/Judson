#!/bin/sh

sudo cp ./judson.service /etc/systemd/system/judson.service
cargo build --release
sudo mv $PWD/target/release/judson /bin
# judson
